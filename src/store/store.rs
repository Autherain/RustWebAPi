//! Structure agrégée du store : chaque champ est le "store" pour une partie du domaine.
//! Équivalent Go : type Store struct { Items ItemRepository; Guests GuestRepository; ... }

use std::sync::Arc;

use crate::domain::{GuestRepository, ItemRepository};
use sqlx::SqlitePool;

use super::guest::SqliteGuestStore;
use super::item::MemoryItemStore;

/// Store agrégé : une structure dont chaque champ satisfait une interface du domaine.
pub struct Store {
    /// Store des items (interface ItemRepository du domaine).
    pub items: Arc<dyn ItemRepository>,
    /// Store des guests (SQLite).
    pub guests: Arc<dyn GuestRepository>,
}

impl Store {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            items: Arc::new(MemoryItemStore::default()),
            guests: Arc::new(SqliteGuestStore::new(pool)),
        }
    }
}

impl Clone for Store {
    fn clone(&self) -> Self {
        Self {
            items: Arc::clone(&self.items),
            guests: Arc::clone(&self.guests),
        }
    }
}
