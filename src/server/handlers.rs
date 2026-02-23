//! Handlers HTTP : orchestration domaine + store + mappers.
//!
//! Spans (contexte de log par requête, style Go `logger.With("request_id", id)`) :
//! on peut attacher un span à toute la durée du handler pour que les logs domaine/store
//! aient automatiquement request_id, item_id, etc. Pour ça : créer un span puis exécuter
//! le corps dans `async move { ... }.instrument(span).await` (voir commentaire dans create_item).
//! Le span ne se perd pas dans les appels domaine/store (même tâche) ; il se perdrait
//! dans un `tokio::spawn` sans `.instrument(span.clone())`.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::domain::{validate_item_name, Item};
use crate::server::dto::{CreateItemRequest, ItemResponse};
use crate::server::error::ApiError;
use crate::server::mapper::{domain_to_response, request_to_name};
use crate::server::state::AppState;

/// GET / — Hello World
pub async fn hello() -> &'static str {
    "Hello World"
}

/// POST /items — Créer un item (validation domaine + repository + mapper).
#[utoipa::path(
    post,
    path = "/items",
    request_body = CreateItemRequest,
    responses(
        (status = 201, description = "Item créé", body = ItemResponse),
        (status = 400, description = "Requête invalide")
    ),
    tag = "items"
)]
pub async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Pour un contexte de log par requête (request_id, item_id dans tous les logs) :
    //   let span = tracing::info_span!("create_item", request_id = %uuid::Uuid::new_v4());
    //   async move { ... tout le corps du handler ... }.instrument(span).await
    let name = request_to_name(&payload);
    validate_item_name(name)?;

    tracing::error!(payload = ?payload, "payload");

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
    params(
        ("id" = String, Path, description = "ID de l'item")
    ),
    responses(
        (status = 200, description = "Item trouvé", body = ItemResponse),
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

#[derive(OpenApi)]
#[openapi(
    paths(create_item, get_item),
    components(schemas(CreateItemRequest, ItemResponse)),
    info(
        title = "Hello World API",
        version = "0.1.0",
        description = "API minimaliste : Hello World + Create/Get en RAM (DDD)"
    ),
    tags((name = "items", description = "Items en mémoire"))
)]
struct ApiDoc;

/// Construit le routeur Axum avec Swagger UI.
/// tower-http : x-request-id (header + extensions) + TraceLayer avec trace_id dans le span
/// pour que tous les logs (handler, domaine, store) aient trace_id dans le JSON.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/items", post(create_item))
        .route("/items/:id", get(get_item))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid::default()))
        .layer(
            TraceLayer::new_for_http().make_span_with(|req: &axum::http::Request<axum::body::Body>| {
                let trace_id = req
                    .extensions()
                    .get::<RequestId>()
                    .and_then(|id: &RequestId| id.header_value().to_str().ok())
                    .map(String::from)
                    .filter(|s: &String| !s.is_empty())
                    .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
                tracing::info_span!("request", trace_id = %trace_id)
            }),
        )
        .layer(PropagateRequestIdLayer::x_request_id())
        .with_state(state)
}
