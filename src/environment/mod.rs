use std::env;

/// Charge le fichier `.env` depuis le répertoire courant ou un parent.
/// On ignore l'erreur si le fichier est absent (comportement proche de godotenv.Load).
#[derive(Debug, Clone)]
pub struct Variables {
    pub nats_url: String,
    /// Chemin ou URL SQLite (ex: `sqlite:./data.db` ou `./data.db`).
    pub database_url: String,
}

fn var_default(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(valeur) => valeur,
        Err(_) => default.to_string(),
    }
}

pub fn parse() -> Variables {
    let _ = dotenvy::dotenv();

    let nats_url = var_default("ECH_NATS_URL", "nats://localhost:4222");
    let database_url = var_default("ECH_DATABASE_URL", "sqlite::memory:");

    Variables {
        nats_url,
        database_url,
    }
}
