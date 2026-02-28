//! Mappers domaine Item ↔ DTOs API.

use crate::domain::Item;
use crate::server::item::dto::{CreateItemRequest, ItemResponse};

/// Domaine → DTO réponse API.
pub fn domain_to_response(item: &Item) -> ItemResponse {
    ItemResponse {
        id: item.id.clone(),
        name: item.name.clone(),
    }
}

/// DTO requête API → nom pour le domaine.
/// La validation du nom est faite côté domaine avant de construire l'Item.
pub fn request_to_name(req: &CreateItemRequest) -> &str {
    req.name.as_str()
}
