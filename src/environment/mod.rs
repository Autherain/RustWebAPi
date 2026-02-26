use std::env;

/// Charge le fichier `.env` depuis le répertoire courant ou un parent.
/// On ignore l'erreur si le fichier est absent (comportement proche de godotenv.Load).
#[derive(Debug, Clone)]
pub struct Variables {
    pub nats_url: String,
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

    Variables {
        nats_url,
    }
}
