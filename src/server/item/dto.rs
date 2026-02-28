//! DTOs API pour les items.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Corps de requête pour créer un item.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateItemRequest {
    pub name: String,
}

/// Réponse API : un item (sérialisé en JSON).
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ItemResponse {
    pub id: String,
    pub name: String,
}
