//! Server HTTP : modules par ressource (item, guest), état, router.

mod error;
mod guest;
mod handlers;
mod item;
mod state;

pub use guest::spawn_guests_stream_tasks;
pub use handlers::router;
pub use state::AppState;