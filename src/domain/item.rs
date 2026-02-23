//! Entité domaine Item.

use serde::{Deserialize, Serialize};

/// Entité domaine : un item avec identifiant et nom.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
}

impl Item {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
