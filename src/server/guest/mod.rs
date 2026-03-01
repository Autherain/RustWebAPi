//! Module serveur pour les guests : DTOs, mappers, handlers, validation, stream NATS.

pub mod dto;
pub mod handlers;
mod mapper;
pub mod stream;
mod validation;

pub use dto::{
    CreateGuestRequest, GuestResponse, StructuredValueBoolInput, StructuredValueBoolResponse,
    StructuredValueStringInput, StructuredValueStringResponse, UpdateGuestRequest,
};
pub use handlers::{create_guest, delete_guest, get_guest, update_guest};
pub use stream::spawn_guests_stream_tasks;
