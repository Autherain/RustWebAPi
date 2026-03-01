//! DTOs API pour les guests.
//!
//! Structs concrets (non génériques) pour que utoipa génère un OpenAPI valide
//! sans $ref vers des primitives (String, bool).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ---- Input (requêtes) ----

/// Champ structuré en entrée, valeur string (first_name, last_name, mail, phone).
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct StructuredValueStringInput {
    pub value: String,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub preferred_at: Option<DateTime<Utc>>,
}

/// Champ structuré en entrée, valeur bool (opt_outs).
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct StructuredValueBoolInput {
    pub value: bool,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[allow(dead_code)] // garde la même forme que String pour l'API
    pub preferred_at: Option<DateTime<Utc>>,
}

/// Corps de requête pour créer un guest.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateGuestRequest {
    pub first_name: StructuredValueStringInput,
    pub last_name: StructuredValueStringInput,
    #[serde(default)]
    pub mail: Option<Vec<StructuredValueStringInput>>,
    #[serde(default)]
    pub phone: Option<Vec<StructuredValueStringInput>>,
    #[serde(default)]
    pub opt_outs: Option<Vec<StructuredValueBoolInput>>,
}

/// Corps de requête pour mettre à jour un guest (champs optionnels).
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateGuestRequest {
    pub first_name: Option<StructuredValueStringInput>,
    pub last_name: Option<StructuredValueStringInput>,
    pub mail: Option<Vec<StructuredValueStringInput>>,
    pub phone: Option<Vec<StructuredValueStringInput>>,
    pub opt_outs: Option<Vec<StructuredValueBoolInput>>,
}

// ---- Response ----

/// Champ structuré en réponse, valeur string.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct StructuredValueStringResponse {
    pub value: String,
    pub from: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub preferred_at: Option<DateTime<Utc>>,
}

/// Champ structuré en réponse, valeur bool.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct StructuredValueBoolResponse {
    pub value: bool,
    pub from: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub preferred_at: Option<DateTime<Utc>>,
}

/// Réponse API : un guest.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct GuestResponse {
    pub id: uuid::Uuid,
    pub first_name: StructuredValueStringResponse,
    pub last_name: StructuredValueStringResponse,
    pub mail: Vec<StructuredValueStringResponse>,
    pub phone: Vec<StructuredValueStringResponse>,
    pub opt_outs: Vec<StructuredValueBoolResponse>,
}
