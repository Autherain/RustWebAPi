//! Server HTTP : DTOs, mappers domaine ↔ API, handlers, état, router.

mod dto;
mod error;
mod handlers;
mod mapper;
mod nats_consumer;
mod state;

pub use handlers::router;
pub use nats_consumer::spawn_guests_stream_tasks;
pub use state::AppState;