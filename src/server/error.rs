//! Erreurs HTTP : conversion domaine → réponse API.
//! Permet d'utiliser `?` dans les handlers pour garder le chemin de succès linéaire.
//! Chaque erreur est loguée (niveau adapté) pour Datadog / agrégation de logs.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::domain::{RepositoryError, ValidationError};

/// Erreur côté API : validation (400), not found (404) ou repository (500).
#[derive(Debug)]
pub enum ApiError {
    Validation(ValidationError),
    Repository(RepositoryError),
    NotFound,
}

impl From<ValidationError> for ApiError {
    fn from(e: ValidationError) -> Self {
        ApiError::Validation(e)
    }
}

impl From<RepositoryError> for ApiError {
    fn from(e: RepositoryError) -> Self {
        ApiError::Repository(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::Validation(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            ApiError::Repository(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
        };

        // Log pour Datadog : error pour 5xx, warn pour 4xx (client / not found / validation)
        match &self {
            ApiError::Repository(e) => {
                tracing::error!(
                    status = %status.as_u16(),
                    error = %e,
                    "api_error: {}",
                    message
                );
            }
            ApiError::Validation(e) => {
                tracing::warn!(
                    status = %status.as_u16(),
                    error = %e,
                    "api_error: {}",
                    message
                );
            }
            ApiError::NotFound => {
                tracing::warn!(status = %status.as_u16(), "api_error: not found");
            }
        }

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
