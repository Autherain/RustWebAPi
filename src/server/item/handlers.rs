//! Handlers HTTP pour les items.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::domain::{validate_item_name, Item};
use crate::server::error::ApiError;
use crate::server::item::mapper::{domain_to_response, request_to_name};
use crate::server::state::AppState;

/// POST /items — Créer un item (validation domaine + repository + mapper).
#[utoipa::path(
    post,
    path = "/items",
    request_body = crate::server::item::dto::CreateItemRequest,
    responses(
        (status = 201, description = "Item créé", body = crate::server::item::dto::ItemResponse),
        (status = 400, description = "Requête invalide")
    ),
    tag = "items"
)]
pub async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<crate::server::item::dto::CreateItemRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let name = request_to_name(&payload);
    validate_item_name(name)?;

    let id = uuid::Uuid::new_v4().to_string();
    let item = Item::new(id.clone(), name.to_owned());
    tracing::info!(item_id = %id, "handler: creating item");
    let created = state.store.items.create(item).await?;

    Ok((StatusCode::CREATED, Json(domain_to_response(&created))))
}

/// GET /items/{id} — Récupérer un item par id.
#[utoipa::path(
    get,
    path = "/items/{id}",
    params(("id" = String, Path, description = "ID de l'item")),
    responses(
        (status = 200, description = "Item trouvé", body = crate::server::item::dto::ItemResponse),
        (status = 404, description = "Item non trouvé")
    ),
    tag = "items"
)]
pub async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!(item_id = %id, "handler: get_item");
    let item = state
        .store
        .items
        .get_by_id(&id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok((StatusCode::OK, Json(domain_to_response(&item))))
}
