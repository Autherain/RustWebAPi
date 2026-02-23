//! État partagé du serveur (injection du Store).

use crate::store::Store;

/// État de l'application : le serveur dépend du Store (items, cache, …).
pub struct AppState {
    pub store: Store,
}

impl AppState {
    pub fn new(store: Store) -> Self {
        Self { store }
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}
