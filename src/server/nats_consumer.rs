//! Consumer NATS JetStream pour le stream `stream.guests.*`.
//! Écoute les messages et affiche le payload ; un task Tokio envoie des messages en boucle.

use async_nats::jetstream::consumer::PullConsumer;
use async_nats::jetstream::stream::Config;
use async_nats::jetstream::Context;
use async_nats::Client;
use bytes::Bytes;
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tracing::{error, info};

const STREAM_NAME: &str = "GUESTS";
const SUBJECT_PREFIX: &str = "stream.guests";
const CONSUMER_NAME: &str = "guests-consumer";

/// Maximum number of messages that can be processed at the same time.
/// The semaphore ensures we never have more than this many tasks in the "process message" section.
const MAX_CONCURRENT_MESSAGE_PROCESSING: usize = 10;

/// Démarre le consumer (écoute des messages) et le publisher (envoi périodique) en tâches Tokio.
pub fn spawn_guests_stream_tasks(client: Client) {
    let js = async_nats::jetstream::new(client.clone());
    let js_pub = js.clone();

    tokio::spawn(async move {
        if let Err(e) = run_consumer(js).await {
            error!("consumer stream.guests.*: {e}");
        }
    });

    tokio::spawn(async move {
        if let Err(e) = run_publisher(js_pub).await {
            error!("publisher stream.guests.*: {e}");
        }
    });
}

async fn run_consumer(js: Context) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _stream = js
        .get_or_create_stream(Config {
            name: STREAM_NAME.to_string(),
            subjects: vec![format!("{SUBJECT_PREFIX}.>")],
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

    info!("consumer NATS: écoute du stream {STREAM_NAME} (subject {SUBJECT_PREFIX}.*)");

    // Semaphore: limits how many messages we process concurrently. This does NOT limit
    // the number of OS threads (those are fixed in Tokio's worker pool). It only limits
    // how many tasks can hold a "permit" at once. Tasks that call acquire() when all
    // permits are taken will yield until another task drops its permit.
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_MESSAGE_PROCESSING));

    let mut messages = consumer
        .stream()
        .max_messages_per_batch(10)
        .messages()
        .await?;

    while let Some(msg) = messages.next().await {
        match msg {
            Ok(m) => {
                let sem = semaphore.clone();
                // Spawn a task per message so we can process up to MAX_CONCURRENT_MESSAGE_PROCESSING
                // messages in parallel. The semaphore caps concurrency; without it we could
                // spawn unbounded tasks.
                tokio::spawn(async move {
                    // Acquire a permit. If MAX_CONCURRENT_MESSAGE_PROCESSING tasks are already
                    // processing, this .await yields until one of them drops its permit.
                    // No OS thread is blocked—the task is just put aside until a permit is free.
                    let _permit = match sem.acquire().await {
                        Ok(p) => p,
                        Err(_) => return, // semaphore was closed
                    };
                    let payload = String::from_utf8_lossy(&m.payload);
                    info!("[consumer] reçu subject={} payload={}", m.subject, payload);
                    if let Err(e) = m.ack().await {
                        error!("[consumer] ack failed: {}", e);
                    }
                    // _permit is dropped here when the task finishes, releasing the permit
                    // so another waiting task (if any) can proceed.
                });
            }
            Err(e) => {
                error!("erreur message: {}", e);
            }
        }
    }

    Ok(())
}

async fn run_publisher(js: Context) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _stream = js
        .get_or_create_stream(Config {
            name: STREAM_NAME.to_string(),
            subjects: vec![format!("{SUBJECT_PREFIX}.>")],
            max_messages: 10_000,
            ..Default::default()
        })
        .await?;

    let mut n: u64 = 0;
    loop {
        n += 1;
        let subject = format!("{SUBJECT_PREFIX}.demo");
        let payload: Bytes = format!("hello guest #{}", n).into();
        if let Err(e) = js.publish(subject.clone(), payload).await {
            error!("publish {}: {}", subject, e);
        } else {
            info!("[publisher] envoyé {} -> hello guest #{}", subject, n);
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
