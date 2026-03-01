//! Entité domaine Guest : uuid + champs structurés (StructuredValue<T>).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Valeur structurée générique : value + provenance + dates (optionnel preferred pour listes mail/phone).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredValue<T> {
    pub value: T,
    #[serde(default)]
    pub from: Option<String>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub preferred_at: Option<DateTime<Utc>>,
}

impl<T> StructuredValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            from: None,
            updated_at: Utc::now(),
            preferred_at: None,
        }
    }

    pub fn with_from(value: T, from: String) -> Self {
        Self {
            value,
            from: Some(from),
            updated_at: Utc::now(),
            preferred_at: None,
        }
    }

    pub fn with_updated_at(value: T, updated_at: DateTime<Utc>) -> Self {
        Self {
            value,
            from: None,
            updated_at,
            preferred_at: None,
        }
    }
}

/// Entité domaine : un invité avec uuid et champs en StructuredValue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Guest {
    pub id: uuid::Uuid,
    pub first_name: StructuredValue<String>,
    pub last_name: StructuredValue<String>,
    pub mail: Vec<StructuredValue<String>>,
    pub phone: Vec<StructuredValue<String>>,
    pub opt_outs: Vec<StructuredValue<bool>>,
}

impl Guest {
    pub fn new(id: uuid::Uuid, first_name: String, last_name: String) -> Self {
        Self {
            id,
            first_name: StructuredValue::new(first_name),
            last_name: StructuredValue::new(last_name),
            mail: Vec::new(),
            phone: Vec::new(),
            opt_outs: Vec::new(),
        }
    }
}
