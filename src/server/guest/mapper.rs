//! Mappers domaine Guest ↔ DTOs API.

use chrono::Utc;

use crate::domain::{Guest, StructuredValue};
use crate::server::guest::dto::{
    CreateGuestRequest, GuestResponse, StructuredValueBoolInput, StructuredValueBoolResponse,
    StructuredValueStringInput, StructuredValueStringResponse, UpdateGuestRequest,
};

fn structured_value_string_to_response(s: &StructuredValue<String>) -> StructuredValueStringResponse {
    StructuredValueStringResponse {
        value: s.value.clone(),
        from: s.from.clone(),
        updated_at: s.updated_at,
        preferred_at: s.preferred_at,
    }
}

fn structured_value_bool_to_response(s: &StructuredValue<bool>) -> StructuredValueBoolResponse {
    StructuredValueBoolResponse {
        value: s.value,
        from: s.from.clone(),
        updated_at: s.updated_at,
        preferred_at: s.preferred_at,
    }
}

fn structured_value_input_to_domain_string(input: StructuredValueStringInput) -> StructuredValue<String> {
    let updated_at = input.updated_at.unwrap_or_else(Utc::now);
    StructuredValue {
        value: input.value,
        from: input.from,
        updated_at,
        preferred_at: input.preferred_at,
    }
}

fn structured_value_input_to_domain_bool(input: StructuredValueBoolInput) -> StructuredValue<bool> {
    let updated_at = input.updated_at.unwrap_or_else(Utc::now);
    StructuredValue {
        value: input.value,
        from: input.from,
        updated_at,
        preferred_at: None,
    }
}

pub fn guest_to_response(guest: &Guest) -> GuestResponse {
    GuestResponse {
        id: guest.id,
        first_name: structured_value_string_to_response(&guest.first_name),
        last_name: structured_value_string_to_response(&guest.last_name),
        mail: guest.mail.iter().map(structured_value_string_to_response).collect(),
        phone: guest.phone.iter().map(structured_value_string_to_response).collect(),
        opt_outs: guest.opt_outs.iter().map(structured_value_bool_to_response).collect(),
    }
}

/// Crée un nouveau Guest à partir de CreateGuestRequest (génère un nouvel uuid).
pub fn create_request_to_guest(req: &CreateGuestRequest) -> Guest {
    let mail = req
        .mail
        .as_ref()
        .map(|v| v.iter().cloned().map(structured_value_input_to_domain_string).collect())
        .unwrap_or_default();
    let phone = req
        .phone
        .as_ref()
        .map(|v| v.iter().cloned().map(structured_value_input_to_domain_string).collect())
        .unwrap_or_default();
    let opt_outs = req
        .opt_outs
        .as_ref()
        .map(|v| v.iter().cloned().map(structured_value_input_to_domain_bool).collect())
        .unwrap_or_default();
    Guest {
        id: uuid::Uuid::new_v4(),
        first_name: structured_value_input_to_domain_string(req.first_name.clone()),
        last_name: structured_value_input_to_domain_string(req.last_name.clone()),
        mail,
        phone,
        opt_outs,
    }
}

/// Applique UpdateGuestRequest sur un guest existant (met à jour les champs fournis).
pub fn apply_update_request(guest: Guest, req: &UpdateGuestRequest) -> Guest {
    let first_name = req
        .first_name
        .as_ref()
        .map(|v| structured_value_input_to_domain_string(v.clone()))
        .unwrap_or(guest.first_name);
    let last_name = req
        .last_name
        .as_ref()
        .map(|v| structured_value_input_to_domain_string(v.clone()))
        .unwrap_or(guest.last_name);
    let mail = req
        .mail
        .as_ref()
        .map(|v| v.iter().cloned().map(structured_value_input_to_domain_string).collect())
        .unwrap_or(guest.mail);
    let phone = req
        .phone
        .as_ref()
        .map(|v| v.iter().cloned().map(structured_value_input_to_domain_string).collect())
        .unwrap_or(guest.phone);
    let opt_outs = req
        .opt_outs
        .as_ref()
        .map(|v| v.iter().cloned().map(structured_value_input_to_domain_bool).collect())
        .unwrap_or(guest.opt_outs);
    Guest {
        id: guest.id,
        first_name,
        last_name,
        mail,
        phone,
        opt_outs,
    }
}
