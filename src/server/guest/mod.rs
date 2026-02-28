//! Module serveur pour les guests : DTOs, mappers, handlers, stream NATS.

pub mod dto;
pub mod handlers;
mod mapper;
pub mod stream;

pub use dto::{CreateGuestRequest, GuestResponse, JsonFieldResponse, UpdateGuestRequest};
pub use handlers::{create_guest, delete_guest, get_guest, update_guest};
pub use mapper::{apply_update_request, create_request_to_guest, guest_to_response};
pub use stream::spawn_guests_stream_tasks;
