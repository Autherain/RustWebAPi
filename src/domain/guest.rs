//! Entité domaine Guest : uuid + nom/prénom en JSON (value, updated_at).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Champ JSON pour un attribut (nom ou prénom) avec valeur et date de mise à jour.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonField {
    pub value: String,
    pub updated_at: DateTime<Utc>,
}

impl JsonField {
    pub fn new(value: String) -> Self {
        Self {
            value,
            updated_at: Utc::now(),
        }
    }

    pub fn with_updated_at(value: String, updated_at: DateTime<Utc>) -> Self {
        Self { value, updated_at }
    }
}

/// Entité domaine : un invité avec uuid et nom/prénom en JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Guest {
    pub id: uuid::Uuid,
    pub first_name: JsonField,
    pub last_name: JsonField,
}

impl Guest {
    pub fn new(id: uuid::Uuid, first_name: String, last_name: String) -> Self {
        Self {
            id,
            first_name: JsonField::new(first_name),
            last_name: JsonField::new(last_name),
        }
    }
}
