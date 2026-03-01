//! Validation des requêtes guest : au plus un préféré par liste, format email, id UUID.

use crate::domain::ValidationError;
use crate::server::guest::dto::{
    CreateGuestRequest, StructuredValueStringInput, UpdateGuestRequest,
};

/// Vérifie qu'au plus un élément a `preferred_at` renseigné.
fn at_most_one_preferred(items: &[StructuredValueStringInput]) -> bool {
    items.iter().filter(|i| i.preferred_at.is_some()).count() <= 1
}

/// Valide le format d'une adresse email (simple : contient @, non vide, longueur raisonnable).
fn is_valid_email_format(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() || s.len() > 254 {
        return false;
    }
    let parts: Vec<&str> = s.split('@').collect();
    parts.len() == 2 && !parts[0].is_empty() && parts[1].contains('.')
}

/// Valide une liste d'emails : au plus un préféré, chaque value au format email.
fn validate_mail_list(mails: &[StructuredValueStringInput]) -> Result<(), ValidationError> {
    if !at_most_one_preferred(mails) {
        return Err(ValidationError(
            "mail: au plus un email peut avoir preferred_at".into(),
        ));
    }
    for (i, m) in mails.iter().enumerate() {
        if !is_valid_email_format(m.value.trim()) {
            return Err(ValidationError(format!(
                "mail[{}]: value doit être une adresse email valide",
                i
            )));
        }
    }
    Ok(())
}

/// Valide une liste de téléphones : au plus un préféré.
fn validate_phone_list(phones: &[StructuredValueStringInput]) -> Result<(), ValidationError> {
    if !at_most_one_preferred(phones) {
        return Err(ValidationError(
            "phone: au plus un numéro peut avoir preferred_at".into(),
        ));
    }
    Ok(())
}

/// Valide qu'une valeur string structurée est non vide (après trim).
fn required_value_non_empty(field: &str, input: &StructuredValueStringInput) -> Result<(), ValidationError> {
    if input.value.trim().is_empty() {
        return Err(ValidationError(format!(
            "{}: value est obligatoire et ne peut pas être vide",
            field
        )));
    }
    Ok(())
}

/// Valide le corps de la requête de création.
pub fn validate_create_request(req: &CreateGuestRequest) -> Result<(), ValidationError> {
    required_value_non_empty("first_name", &req.first_name)?;
    required_value_non_empty("last_name", &req.last_name)?;
    if let Some(ref mails) = req.mail {
        validate_mail_list(mails)?;
    }
    if let Some(ref phones) = req.phone {
        validate_phone_list(phones)?;
    }
    Ok(())
}

/// Valide le corps de la requête de mise à jour.
pub fn validate_update_request(req: &UpdateGuestRequest) -> Result<(), ValidationError> {
    if let Some(ref first) = req.first_name {
        required_value_non_empty("first_name", first)?;
    }
    if let Some(ref last) = req.last_name {
        required_value_non_empty("last_name", last)?;
    }
    if let Some(ref mails) = req.mail {
        validate_mail_list(mails)?;
    }
    if let Some(ref phones) = req.phone {
        validate_phone_list(phones)?;
    }
    Ok(())
}

/// Parse l'id path en UUID ; retourne une ValidationError si le format est invalide (pour 400).
pub fn parse_guest_id(id: &str) -> Result<uuid::Uuid, ValidationError> {
    uuid::Uuid::parse_str(id).map_err(|_| {
        ValidationError(format!("id invalide: '{}' n'est pas un UUID valide", id))
    })
}
