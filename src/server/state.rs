//! État partagé du serveur (injection du Store et du client NATS).

use crate::store::Store;
use async_nats::Client;

/// État de l'application : le serveur dépend du Store et du client NATS.
pub struct AppState {
    pub store: Store,
    pub nats: Client,
}

impl AppState {
    pub fn new(store: Store, nats: Client) -> Self {
        Self { store, nats }
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            nats: self.nats.clone(),
        }
    }
}
