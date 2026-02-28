//! Mappers domaine Guest ↔ DTOs API.

use chrono::Utc;

use crate::domain::{Guest, JsonField};
use crate::server::guest::dto::{
    CreateGuestRequest, GuestResponse, JsonFieldResponse, UpdateGuestRequest,
};

fn json_field_to_response(f: &JsonField) -> JsonFieldResponse {
    JsonFieldResponse {
        value: f.value.clone(),
        updated_at: f.updated_at,
    }
}

pub fn guest_to_response(guest: &Guest) -> GuestResponse {
    GuestResponse {
        id: guest.id,
        first_name: json_field_to_response(&guest.first_name),
        last_name: json_field_to_response(&guest.last_name),
    }
}

/// Crée un nouveau Guest à partir de CreateGuestRequest (génère un nouvel uuid).
pub fn create_request_to_guest(req: &CreateGuestRequest) -> Guest {
    Guest::new(
        uuid::Uuid::new_v4(),
        req.first_name.clone(),
        req.last_name.clone(),
    )
}

/// Applique UpdateGuestRequest sur un guest existant (met à jour les champs fournis avec updated_at = now).
pub fn apply_update_request(guest: Guest, req: &UpdateGuestRequest) -> Guest {
    let first_name = req
        .first_name
        .as_ref()
        .map(|v| JsonField::with_updated_at(v.clone(), Utc::now()))
        .unwrap_or(guest.first_name);
    let last_name = req
        .last_name
        .as_ref()
        .map(|v| JsonField::with_updated_at(v.clone(), Utc::now()))
        .unwrap_or(guest.last_name);
    Guest {
        id: guest.id,
        first_name,
        last_name,
    }
}
