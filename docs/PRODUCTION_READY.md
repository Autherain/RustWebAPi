# Production-ready : config, panic, shutdown, logging, mocking

Ce document décrit comment rendre l’API prête pour la prod : configuration par variables d’environnement, gestion des erreurs sans panic, arrêt gracieux, logging structuré et mocking pour les tests.

---

## 1. Configuration (env + valeurs par défaut, struct style Go)

**Objectif** : un type `Config` (comme en Go) dont les champs viennent des variables d’environnement, avec des valeurs par défaut si absentes.

**Librairie recommandée** : **`config`** ou rester en std avec **`std::env::var`**.

- **Option simple (sans crate)** : lire `std::env::var("KEY")` et utiliser `.unwrap_or_else(|_| "default".to_string())` ou un helper.
- **Option structurée** : crate **`config`** (fichiers + env + hiérarchie) ou **`envy`** (uniquement env → struct).

Exemple **sans crate** (struct + env + défauts) :

```rust
// config.rs
pub struct Config {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3000),
            log_level: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        }
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
```

**Avec une crate** (ex. **`config`** + **`serde`**) on peut mélanger fichier `config.toml` et override par env (style `CONFIG__SERVER__PORT=4000`). Pour commencer, le snippet ci-dessus suffit.

**Crates utiles** :
- [config](https://crates.io/crates/config) — fichier + env, hiérarchie.
- [envy](https://crates.io/crates/envy) — env → struct serde (une seule source).

---

## 2. Éviter les panic en `main` (bind, serve)

**Problème** : `expect("bind")` et `expect("serve")` font paniquer le process si le bind ou le serve échoue.

**Principe** : faire que `main` retourne un `Result` (ou utiliser `std::process::exit`) et ne jamais appeler `expect` / `unwrap` pour des erreurs récupérables.

**Pattern recommandé** : `main` async qui retourne `Result<(), Box<dyn std::error::Error>>`, puis dans le binaire :

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

En cas d’erreur, l’erreur est propagée, le process quitte avec un code non nul et le message est affiché. Pour un message plus lisible, tu peux faire :

```rust
if let Err(e) = run().await {
    eprintln!("Fatal: {}", e);
    std::process::exit(1);
}
```

**Pas besoin de librairie spéciale** : le type `Result` et `?` en Rust suffisent. Éviter `expect()` / `unwrap()` dans les chemins d’erreur récupérables (bind, serve, lecture config).

---

## 3. Graceful shutdown (arrêt propre)

**Objectif** : à la réception de SIGTERM / SIGINT (Ctrl+C), arrêter d’accepter de nouvelles requêtes, laisser finir celles en cours, puis quitter.

**Comment** : `axum::serve` accepte un second argument optionnel : un **future** qui se résout quand on veut arrêter le serveur. On utilise en général les signaux tokio.

**Librairie** : rien à ajouter, **`tokio`** fournit déjà `tokio::signal`.

Exemple :

```rust
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let app = router(state);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

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
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
```

Dès que l’utilisateur fait Ctrl+C ou que le process reçoit SIGTERM, `shutdown_signal()` se résout, `axum::serve` arrête proprement puis `main` se termine.

---

## 4. Logging structuré

**Objectif** : remplacer les `println!` par un vrai logging (niveaux, format structuré, compatible avec la prod).

**Librairie recommandée** : **`tracing`** + **`tracing-subscriber`**.

- **`tracing`** : utilisé par tokio / axum ; macros `info!`, `error!`, `warn!`, `debug!` ; support du contexte (spans).
- **`tracing-subscriber`** : enregistre les événements (format human ou JSON, niveau filtré par `RUST_LOG`).

**Cargo.toml** :

```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

**Initialisation en début de `main`** (après avoir lu la config si tu veux un niveau configurable) :

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn init_logging(log_level: Option<&str>) {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::new(log_level.unwrap_or("info")));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
```

**Utilisation** :

```rust
tracing::info!("API démarrée sur http://{}", config.bind_addr());
tracing::error!("bind failed: {}", e);
```

La variable d’environnement **`RUST_LOG`** contrôle le niveau (ex. `RUST_LOG=debug`, `RUST_LOG=hello_world_api=debug`). Pas besoin d’autre lib pour un usage standard.

---

## 5. Mocking pour les tests (ItemRepository)

**Objectif** : tester les handlers (ou la logique qui appelle le repository) sans le vrai store — en injectant une implémentation “fake” du trait **`ItemRepository`**.

**Deux approches** :

### A) Mock manuel (sans crate)

Tu gardes le trait `ItemRepository` et tu crées une struct de test qui l’implémente (en mémoire, comportement contrôlé) :

```rust
// tests ou src/server/handlers.rs #[cfg(test)]
struct MockItemRepository {
    items: std::sync::Mutex<HashMap<String, Item>>,
    fail_create: bool,
    fail_get_by_id: bool,
}

#[async_trait]
impl ItemRepository for MockItemRepository {
    async fn create(&self, item: Item) -> Result<Item, RepositoryError> {
        if self.fail_create { return Err(RepositoryError::Other("mock".into())); }
        let id = item.id().to_string();
        self.items.lock().unwrap().insert(id.clone(), item.clone());
        Ok(item)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Item>, RepositoryError> {
        if self.fail_get_by_id { return Err(RepositoryError::Other("mock".into())); }
        Ok(self.items.lock().unwrap().get(id).cloned())
    }
}
```

Pour que les handlers utilisent ce mock, il faut que **`AppState`** (ou le type passé en `State`) dépende du **trait** (ex. `dyn ItemRepository` ou générique) et non du `Store` concret. Tu construis alors un `AppState` avec `MockItemRepository` dans les tests.

### B) Génération de mocks avec **mockall**

**Crate** : **`mockall`** — génère une implémentation “mock” à partir d’un trait (enregistrer des attentes, retourner des valeurs définies).

**Cargo.toml** (dev) :

```toml
[dev-dependencies]
mockall = "0.13"
```

**Limitation** : `mockall` génère des mocks pour des traits dont les méthodes sont **sync**. Ton `ItemRepository` est **async** (`async_trait`). Pour async, mockall peut marcher avec `async_trait::async_trait` en gardant la même signature, mais il faut vérifier la compatibilité (versions). Sinon le mock manuel (A) est la voie la plus simple avec async.

**Résumé** :
- **Tests d’intégration** : garder `AppState` avec le vrai `Store`, appeler l’API via `axum::test::TestClient` — pas de mock, tout est réel.
- **Tests unitaires de la logique qui appelle le repo** : injecter un **trait** (ex. `ItemRepository`) et utiliser un **mock manuel** ou un type in-memory dédié aux tests.

---

## Récap des crates suggérées

| Besoin       | Crate(s)                    | Remarque                          |
|-------------|-----------------------------|-----------------------------------|
| Config      | `std::env` ou `config`/`envy` | Struct + env + défauts, comme en Go. |
| Panic       | Aucune                      | `Result` + `?` + `exit(1)` en main. |
| Shutdown    | `tokio` (déjà présent)     | `axum::serve(...).with_graceful_shutdown(signal())`. |
| Logging     | `tracing`, `tracing-subscriber` | Standard écosystème tokio/axum.   |
| Mocking     | Mock manuel ou `mockall`   | Async : mock manuel souvent plus simple. |

---

## Ordre d’implémentation suggéré

1. **Config** : struct `Config` + `from_env()` + utilisation dans `main` (bind_addr, optionnellement niveau de log).
2. **Logging** : `tracing-subscriber` en début de `main`, remplacer `println!` par `tracing::info!` etc.
3. **Panic** : `main` retourne `Result<(), Box<dyn Error>>`, remplacer `expect` par `?` et optionnellement `eprintln` + `exit(1)`.
4. **Shutdown** : `with_graceful_shutdown(shutdown_signal())` sur `axum::serve`.
5. **Tests** : tests unitaires sur la validation ; tests d’intégration sur les handlers ; si besoin, introduire un trait pour le repository et un mock manuel pour des tests unitaires ciblés.

Une fois tout ça en place, l’API est bien plus proche du “production-ready” (config, logs, arrêt propre, pas de panic en chemin critique, tests et mocking possibles).
