//! Tout ce qui est propre aux items dans le store : type persistant, mappers, implémentation.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::{Item, ItemRepository, RepositoryError};

// ---------- Type de données du store (représentation persistance) ----------

/// Représentation d'un item telle que stockée en mémoire/DB.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemRow {
    pub id: String,
    pub name: String,
}

impl ItemRow {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}

// ---------- Mappers domaine ↔ store ----------

fn domain_to_row(item: &Item) -> ItemRow {
    ItemRow::new(item.id.clone(), item.name.clone())
}

fn row_to_domain(row: &ItemRow) -> Item {
    Item::new(row.id.clone(), row.name.clone())
}

// ---------- Implémentation en RAM du ItemRepository (interface du domaine) ----------

/// Store en mémoire pour les items : satisfait l'interface ItemRepository.
pub struct MemoryItemStore {
    inner: Arc<RwLock<HashMap<String, ItemRow>>>,
}

impl Default for MemoryItemStore {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::<String, ItemRow>::new())),
        }
    }
}

#[async_trait]
impl ItemRepository for MemoryItemStore {
    async fn create(&self, item: Item) -> Result<Item, RepositoryError> {
        let row = domain_to_row(&item);
        let id = row.id.clone();
        self.inner.write().await.insert(id.clone(), row.clone());
        tracing::info!(item_id = %id, name = %row.name, "store: item created");
        Ok(row_to_domain(&row))
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Item>, RepositoryError> {
        let guard = self.inner.read().await;
        let found = guard.get(id).map(row_to_domain);
        tracing::debug!(item_id = %id, found = found.is_some(), "store: get_by_id");
        Ok(found)
    }
}
