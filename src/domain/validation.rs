//! Règles de validation du domaine.

use std::fmt;

/// Erreur de validation métier.
#[derive(Debug, Clone)]
pub struct ValidationError(pub String);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ValidationError {}

/// Valide que le nom d'un item est non vide et pas trop long.
pub fn validate_item_name(name: &str) -> Result<(), ValidationError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(ValidationError("name must be non-empty".into()));
    }
    if name.len() > 500 {
        return Err(ValidationError("name must be at most 500 characters".into()));
    }
    Ok(())
}
