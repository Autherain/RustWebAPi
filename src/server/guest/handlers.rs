//! Handlers HTTP pour les guests.

use async_nats::jetstream::context::traits::Publisher;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use tower_http::request_id::RequestId;

use crate::server::error::ApiError;
use crate::server::guest::dto::{CreateGuestRequest, UpdateGuestRequest};
use crate::server::guest::mapper::{
    apply_update_request, create_request_to_guest, guest_to_response,
};
use crate::server::guest::validation::{
    parse_guest_id, validate_create_request, validate_update_request,
};
use crate::server::state::AppState;

/// POST /guests — Créer un guest.
#[utoipa::path(
    post,
    path = "/guests",
    request_body = crate::server::guest::dto::CreateGuestRequest,
    responses(
        (status = 201, description = "Guest créé", body = crate::server::guest::dto::GuestResponse),
        (status = 400, description = "Requête invalide (ex: au plus un email/téléphone préféré, format email)")
    ),
    tag = "guests"
)]
pub async fn create_guest(
    State(state): State<AppState>,
    Json(payload): Json<CreateGuestRequest>,
) -> Result<impl IntoResponse, ApiError> {
    validate_create_request(&payload)?;
    let guest = create_request_to_guest(&payload);
    tracing::info!(guest_id = %guest.id, "handler: creating guest");
    let created = state.store.guests.create(guest).await?;
    Ok((StatusCode::CREATED, Json(guest_to_response(&created))))
}

/// GET /guests/{id} — Récupérer un guest par uuid.
#[utoipa::path(
    get,
    path = "/guests/{id}",
    params(("id" = String, Path, description = "UUID du guest")),
    responses(
        (status = 200, description = "Guest trouvé", body = crate::server::guest::dto::GuestResponse),
        (status = 400, description = "Id invalide (format UUID)"),
        (status = 404, description = "Guest non trouvé")
    ),
    tag = "guests"
)]
pub async fn get_guest(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let uuid = parse_guest_id(&id)?;
    let guest = state
        .store
        .guests
        .get_by_id(&uuid)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok((StatusCode::OK, Json(guest_to_response(&guest))))
}

/// PUT /guests/{id} — Mettre à jour un guest.
#[utoipa::path(
    put,
    path = "/guests/{id}",
    params(("id" = String, Path, description = "UUID du guest")),
    request_body = crate::server::guest::dto::UpdateGuestRequest,
    responses(
        (status = 200, description = "Guest mis à jour", body = crate::server::guest::dto::GuestResponse),
        (status = 400, description = "Requête invalide (ex: id invalide, au plus un email/téléphone préféré)"),
        (status = 404, description = "Guest non trouvé")
    ),
    tag = "guests"
)]
pub async fn update_guest(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateGuestRequest>,
) -> Result<impl IntoResponse, ApiError> {
    validate_update_request(&payload)?;
    let uuid = parse_guest_id(&id)?;
    let existing = state
        .store
        .guests
        .get_by_id(&uuid)
        .await?
        .ok_or(ApiError::NotFound)?;
    let updated = apply_update_request(existing, &payload);
    let saved = state.store.guests.update(updated).await?;
    Ok((StatusCode::OK, Json(guest_to_response(&saved))))
}

/// DELETE /guests/{id} — Supprimer un guest et publier un message opt-out sur NATS.
#[utoipa::path(
    delete,
    path = "/guests/{id}",
    params(("id" = String, Path, description = "UUID du guest")),
    responses(
        (status = 204, description = "Guest supprimé"),
        (status = 400, description = "Id invalide (format UUID)"),
        (status = 404, description = "Guest non trouvé")
    ),
    tag = "guests"
)]
pub async fn delete_guest(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let uuid = parse_guest_id(&id)?;
    let deleted = state.store.guests.delete(&uuid).await?;
    let deleted_id = deleted.ok_or(ApiError::NotFound)?;

    let trace_id = request_id
        .header_value()
        .to_str()
        .unwrap_or("")
        .to_string();

    let outbound = async_nats::jetstream::message::PublishMessage::build()
        .header(super::stream::TRACE_ID_HEADER, trace_id.as_str())
        .payload(bytes::Bytes::from(super::stream::OPT_OUT_MESSAGE))
        .outbound_message(super::stream::OPT_OUT_SUBJECT);

    let js = async_nats::jetstream::new(state.nats.clone());
    if let Err(e) = js.publish_message(outbound).await {
        tracing::warn!(guest_id = %deleted_id, subject = super::stream::OPT_OUT_SUBJECT, "publish opt-out: {}", e);
    } else {
        tracing::info!(guest_id = %deleted_id, "published opt-out to {}", super::stream::OPT_OUT_SUBJECT);
    }

    Ok(StatusCode::NO_CONTENT)
}
