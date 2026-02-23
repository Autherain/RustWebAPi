//! Interface (trait) du stockage des items — équivalent Go interface.
//! Les implémentations (store) vivent dans `pkg/store` / `store`.

use async_trait::async_trait;

use crate::domain::Item;

/// Erreur retournée par le repository.
#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
    Other(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::NotFound(id) => write!(f, "item not found: {}", id),
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
