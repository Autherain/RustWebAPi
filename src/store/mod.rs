//! Store : structure agrégée + implémentations des interfaces du domaine.
//! Chaque partie (items, guests, …) vit dans son module ; Store les regroupe.

mod guest;
mod item;
mod store;

pub use store::Store;
