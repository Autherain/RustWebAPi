//! Domaine : entités, règles de validation et interfaces (traits).
//! Équivalent du root Go : types du domaine + validators + interfaces.

mod guest;
mod item;
mod repository;
mod validation;

pub use guest::{Guest, StructuredValue};
pub use item::Item;
pub use repository::{GuestRepository, ItemRepository, RepositoryError};
pub use validation::{validate_item_name, ValidationError};