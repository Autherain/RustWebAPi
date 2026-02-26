//! Point d'entrée : wiring domaine → store → server (style DDD, équivalent cmd/server en Go).

use hello_world_api::environment;
use hello_world_api::server::{router, spawn_guests_stream_tasks, AppState};
use hello_world_api::store::Store;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    let env_vars = environment::parse();

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let nats = async_nats::connect(&env_vars.nats_url)
        .await
        .expect("connexion NATS (démarre le container avec: docker compose up -d)");

    spawn_guests_stream_tasks(nats.clone());

    let store = Store::new();
    let state = AppState::new(store, nats);

    let app = router(state);

    let addr = "127.0.0.1:4000";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("serve");
}

/// Future qui se résout à la réception de Ctrl+C ou SIGTERM (Unix).
/// Permet à `axum::serve` d'arrêter proprement (fin des requêtes en cours).
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to listen for Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to listen for SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("signal: Ctrl+C"),
        _ = terminate => tracing::info!("signal: SIGTERM"),
    }
}
