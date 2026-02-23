//! Domaine : entités, règles de validation et interfaces (traits).
//! Équivalent du root Go : types du domaine + validators + interfaces.

mod item;
mod repository;
mod validation;

pub use item::Item;
pub use repository::{ItemRepository, RepositoryError};
pub use validation::{validate_item_name, ValidationError};