//! Point d'entrée : wiring domaine → store → server (style DDD, équivalent cmd/server en Go).

mod domain;
mod server;
mod store;

use server::{router, AppState};
use store::Store;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    // Un seul subscriber global : pas besoin de "logger::default()" ni de passer un logger.
    // Dans store, domain, server : appelez directement tracing::info!, tracing::error!, etc.
    // Les "attributs" sont les champs structurés : tracing::info!(user_id = 42, "msg").
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let store = Store::new();
    let state = AppState::new(store);

    let app = router(state);

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");

    tracing::info!(%addr, "API démarrée");
    tracing::debug!(
        "routes: GET /, POST /items, GET /items/:id, GET /swagger-ui"
    );

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
