//! Store : structure agrégée + implémentations des interfaces du domaine.
//! Chaque partie (items, cache, …) vit dans son module ; Store les regroupe.

mod item;
mod store;

pub use store::Store;
