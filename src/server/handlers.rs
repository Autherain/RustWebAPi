//! Router HTTP et point d’entrée des handlers.
//!
//! Middlewares (ServiceBuilder) : TraceLayer → Timeout → ConcurrencyLimit → RequestId → Routes.
//! Les handlers par ressource (items, guests) sont dans leurs modules dédiés.

use axum::{
    routing::get,
    Router,
};
use std::time::Duration;
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    timeout::TimeoutLayer as HttpTimeoutLayer,
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::server::guest::{create_guest, delete_guest, get_guest, update_guest};
use crate::server::item::{create_item, get_item};
use crate::server::state::AppState;

/// GET / — Hello World
pub async fn hello() -> &'static str {
    "Hello World"
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::server::item::handlers::create_item,
        crate::server::item::handlers::get_item,
        crate::server::guest::handlers::create_guest,
        crate::server::guest::handlers::get_guest,
        crate::server::guest::handlers::update_guest,
        crate::server::guest::handlers::delete_guest,
    ),
    components(schemas(
        crate::server::item::CreateItemRequest,
        crate::server::item::ItemResponse,
        crate::server::guest::CreateGuestRequest,
        crate::server::guest::UpdateGuestRequest,
        crate::server::guest::GuestResponse,
        crate::server::guest::StructuredValueStringInput,
        crate::server::guest::StructuredValueStringResponse,
        crate::server::guest::StructuredValueBoolInput,
        crate::server::guest::StructuredValueBoolResponse,
    )),
    info(
        title = "Hello World API",
        version = "0.1.0",
        description = "API minimaliste : Hello World + Items en RAM + Guests SQLite (DDD)"
    ),
    tags(
        (name = "items", description = "Items en mémoire"),
        (name = "guests", description = "Guests en SQLite")
    )
)]
struct ApiDoc;

/// Construit le routeur Axum avec Swagger UI.
pub fn router(state: AppState) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http().make_span_with(
                |req: &axum::http::Request<axum::body::Body>| {
                    let trace_id = req
                        .extensions()
                        .get::<RequestId>()
                        .and_then(|id: &RequestId| id.header_value().to_str().ok())
                        .map(String::from)
                        .filter(|s: &String| !s.is_empty())
                        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
                    tracing::info_span!("request", trace_id = %trace_id)
                },
            ),
        )
        .layer(HttpTimeoutLayer::with_status_code(
            axum::http::StatusCode::GATEWAY_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(ConcurrencyLimitLayer::new(100))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid::default()))
        .layer(PropagateRequestIdLayer::x_request_id());

    Router::new()
        .route("/", get(hello))
        .route("/items", axum::routing::post(create_item))
        .route("/items/:id", get(get_item))
        .route("/guests", axum::routing::post(create_guest))
        .route(
            "/guests/:id",
            get(get_guest)
                .put(update_guest)
                .delete(delete_guest),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(middleware)
        .with_state(state)
}
