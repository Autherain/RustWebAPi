//! DTOs API pour les guests.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Corps de requête pour créer un guest.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateGuestRequest {
    pub first_name: String,
    pub last_name: String,
}

/// Corps de requête pour mettre à jour un guest (champs optionnels).
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateGuestRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

/// Champ JSON exposé dans l'API (value + updated_at).
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct JsonFieldResponse {
    pub value: String,
    pub updated_at: DateTime<Utc>,
}

/// Réponse API : un guest.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct GuestResponse {
    pub id: uuid::Uuid,
    pub first_name: JsonFieldResponse,
    pub last_name: JsonFieldResponse,
}
