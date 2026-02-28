//! Interface (trait) du stockage des items et guests — équivalent Go interface.
//! Les implémentations (store) vivent dans `pkg/store` / `store`.

use async_trait::async_trait;

use crate::domain::{Guest, Item};

/// Erreur retournée par le repository.
#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
    Other(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::NotFound(id) => write!(f, "not found: {}", id),
            RepositoryError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

/// Interface du store d'items (équivalent Go : type ItemRepository interface { ... }).
#[async_trait]
pub trait ItemRepository: Send + Sync {
    /// Crée un item et le persiste.
    async fn create(&self, item: Item) -> Result<Item, RepositoryError>;

    /// Récupère un item par id.
    async fn get_by_id(&self, id: &str) -> Result<Option<Item>, RepositoryError>;
}

/// Interface du store des guests.
#[async_trait]
pub trait GuestRepository: Send + Sync {
    /// Crée un guest et le persiste.
    async fn create(&self, guest: Guest) -> Result<Guest, RepositoryError>;

    /// Récupère un guest par uuid.
    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Option<Guest>, RepositoryError>;

    /// Met à jour un guest.
    async fn update(&self, guest: Guest) -> Result<Guest, RepositoryError>;

    /// Supprime un guest par uuid. Retourne l'uuid si supprimé.
    async fn delete(&self, id: &uuid::Uuid) -> Result<Option<uuid::Uuid>, RepositoryError>;
}
