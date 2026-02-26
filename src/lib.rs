//! Bibliothèque partagée : domaine, store, server.
//! Les binaires dans `src/cmd/` (ou `src/bin/`) utilisent cette lib pour démarrer l’API ou d’autres exécutables.

pub mod domain;
pub mod environment;
pub mod server;
pub mod store;
