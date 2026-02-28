//! Stream NATS JetStream pour les guests : constantes du sujet et consumer.
//! Écoute les messages (ex. opt-out à la suppression d'un guest) et affiche le payload.

use async_nats::jetstream::consumer::PullConsumer;
use async_nats::jetstream::stream::Config;
use async_nats::jetstream::Context;
use async_nats::Client;
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tracing::{error, info};

// --- Constantes exposées (handler et consumer) ---

/// Sujet sur lequel publier l’événement opt-out lors d’un DELETE guest.
pub const OPT_OUT_SUBJECT: &str = "stream.guest.opt-out";

/// Message envoyé sur OPT_OUT_SUBJECT lors d’un opt-out.
pub const OPT_OUT_MESSAGE: &str = "Guest opt-out from sparkpost";

/// Header NATS pour propager le trace_id (request_id HTTP) jusqu'au consumer.
pub const TRACE_ID_HEADER: &str = "trace-id";

// --- Config stream / consumer ---

/// Stream dédié opt-out (évite les conflits avec un ancien stream GUESTS mal configuré).
const STREAM_NAME: &str = "GUEST_OPT_OUT";
/// Sujet du stream : stream.guest.* (inclut OPT_OUT_SUBJECT).
const STREAM_SUBJECTS: [&str; 1] = ["stream.guest.>"];
const CONSUMER_NAME: &str = "guest-opt-out-consumer";
const MAX_CONCURRENT_MESSAGE_PROCESSING: usize = 10;
/// Durée d'attente max par batch de pull (évite que le consumer s’arrête si aucun message).
const PULL_EXPIRES: Duration = Duration::from_secs(30);

/// Démarre le consumer (écoute des messages du stream) en tâche Tokio.
pub fn spawn_guests_stream_tasks(client: Client) {
    let js = async_nats::jetstream::new(client);

    tokio::spawn(async move {
        if let Err(e) = run_consumer(js).await {
            error!("consumer stream {STREAM_NAME}: {e}");
        }
    });
}

async fn run_consumer(js: Context) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _stream = js
        .get_or_create_stream(Config {
            name: STREAM_NAME.to_string(),
            subjects: STREAM_SUBJECTS.iter().map(|s| (*s).to_string()).collect(),
            max_messages: 10_000,
            ..Default::default()
        })
        .await?;

    let consumer: PullConsumer = js
        .create_consumer_on_stream(
            async_nats::jetstream::consumer::pull::Config {
                durable_name: Some(CONSUMER_NAME.to_string()),
                ..Default::default()
            },
            STREAM_NAME,
        )
        .await?;

    info!(
        "consumer NATS guest: démarré, stream={STREAM_NAME} subject={}",
        STREAM_SUBJECTS[0]
    );

    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_MESSAGE_PROCESSING));

    loop {
        let mut messages = consumer
            .stream()
            .max_messages_per_batch(10)
            .expires(PULL_EXPIRES)
            .messages()
            .await?;

        while let Some(res) = messages.next().await {
            match res {
                Ok(m) => {
                    let sem = semaphore.clone();
                    tokio::spawn(async move {
                        let _permit = match sem.acquire().await {
                            Ok(p) => p,
                            Err(_) => return,
                        };
                        let payload = String::from_utf8_lossy(&m.payload);
                        info!(subject = %m.subject, payload = %payload, "[consumer guest] opt-out reçu");
                        if let Err(e) = m.ack().await {
                            error!("[consumer guest] ack failed: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("[consumer guest] erreur message: {}", e);
                }
            }
        }
        // Batch terminé (timeout ou plus de messages) → on relance un nouveau pull.
    }
}
