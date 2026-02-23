//! Server HTTP : DTOs, mappers domaine ↔ API, handlers, état, router.

mod dto;
mod error;
mod handlers;
mod mapper;
mod state;

pub use handlers::router;
pub use state::AppState;