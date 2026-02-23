//! Structure agrégée du store : chaque champ est le "store" pour une partie du domaine.
//! Équivalent Go : type Store struct { Items ItemRepository; Cache ... }

use std::sync::Arc;

use crate::domain::ItemRepository;

use super::item::MemoryItemStore;

/// Store agrégé : une structure dont chaque champ satisfait une interface du domaine.
pub struct Store {
    /// Store des items (interface ItemRepository du domaine).
    pub items: Arc<dyn ItemRepository>,
    // pub cache: Arc<dyn cache::Cache>,  // exemple pour plus tard
}

impl Store {
    pub fn new() -> Self {
        Self {
            items: Arc::new(MemoryItemStore::default()),
        }
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Store {
    fn clone(&self) -> Self {
        Self {
            items: Arc::clone(&self.items),
        }
    }
}
