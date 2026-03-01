//! Store SQLite pour les guests : implémentation de GuestRepository.

use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};

use crate::domain::{Guest, GuestRepository, RepositoryError, StructuredValue};

/// Row telle que lue depuis SQLite (id + JSON en texte).
#[derive(Debug, FromRow)]
struct GuestRow {
    id: String,
    first_name: String,
    last_name: String,
    mail: String,
    phone: String,
    opt_outs: String,
}

#[async_trait]
impl GuestRepository for SqliteGuestStore {
    async fn create(&self, guest: Guest) -> Result<Guest, RepositoryError> {
        let id = guest.id.to_string();
        let first_name_json =
            serde_json::to_string(&guest.first_name).map_err(|e| RepositoryError::Other(e.to_string()))?;
        let last_name_json =
            serde_json::to_string(&guest.last_name).map_err(|e| RepositoryError::Other(e.to_string()))?;
        let mail_json =
            serde_json::to_string(&guest.mail).map_err(|e| RepositoryError::Other(e.to_string()))?;
        let phone_json =
            serde_json::to_string(&guest.phone).map_err(|e| RepositoryError::Other(e.to_string()))?;
        let opt_outs_json =
            serde_json::to_string(&guest.opt_outs).map_err(|e| RepositoryError::Other(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO guests (id, first_name, last_name, mail, phone, opt_outs)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&first_name_json)
        .bind(&last_name_json)
        .bind(&mail_json)
        .bind(&phone_json)
        .bind(&opt_outs_json)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::Other(e.to_string()))?;

        tracing::info!(guest_id = %guest.id, "store: guest created");
        Ok(guest)
    }

    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Option<Guest>, RepositoryError> {
        let id_str = id.to_string();
        let row = sqlx::query_as::<_, GuestRow>(
            "SELECT id, first_name, last_name, mail, phone, opt_outs FROM guests WHERE id = ?",
        )
        .bind(&id_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::Other(e.to_string()))?;

        let guest = row.map(|r| r.into_guest()).transpose()
            .map_err(|e| RepositoryError::Other(e))?;
        tracing::debug!(guest_id = %id, found = guest.is_some(), "store: guest get_by_id");
        Ok(guest)
    }

    async fn update(&self, guest: Guest) -> Result<Guest, RepositoryError> {
        let id = guest.id.to_string();
        let first_name_json =
            serde_json::to_string(&guest.first_name).map_err(|e| RepositoryError::Other(e.to_string()))?;
        let last_name_json =
            serde_json::to_string(&guest.last_name).map_err(|e| RepositoryError::Other(e.to_string()))?;
        let mail_json =
            serde_json::to_string(&guest.mail).map_err(|e| RepositoryError::Other(e.to_string()))?;
        let phone_json =
            serde_json::to_string(&guest.phone).map_err(|e| RepositoryError::Other(e.to_string()))?;
        let opt_outs_json =
            serde_json::to_string(&guest.opt_outs).map_err(|e| RepositoryError::Other(e.to_string()))?;

        let result = sqlx::query(
            r#"
            UPDATE guests SET first_name = ?, last_name = ?, mail = ?, phone = ?, opt_outs = ? WHERE id = ?
            "#,
        )
        .bind(&first_name_json)
        .bind(&last_name_json)
        .bind(&mail_json)
        .bind(&phone_json)
        .bind(&opt_outs_json)
        .bind(&id)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::Other(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound(id));
        }
        tracing::info!(guest_id = %guest.id, "store: guest updated");
        Ok(guest)
    }

    async fn delete(&self, id: &uuid::Uuid) -> Result<Option<uuid::Uuid>, RepositoryError> {
        let id_str = id.to_string();
        let result = sqlx::query("DELETE FROM guests WHERE id = ?")
            .bind(&id_str)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::Other(e.to_string()))?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            tracing::info!(guest_id = %id, "store: guest deleted");
            Ok(Some(*id))
        } else {
            Ok(None)
        }
    }
}

impl GuestRow {
    fn into_guest(self) -> Result<Guest, String> {
        let id = uuid::Uuid::parse_str(&self.id).map_err(|e| e.to_string())?;
        let first_name: StructuredValue<String> =
            serde_json::from_str(&self.first_name).map_err(|e| e.to_string())?;
        let last_name: StructuredValue<String> =
            serde_json::from_str(&self.last_name).map_err(|e| e.to_string())?;
        let mail: Vec<StructuredValue<String>> =
            serde_json::from_str(&self.mail).map_err(|e| e.to_string())?;
        let phone: Vec<StructuredValue<String>> =
            serde_json::from_str(&self.phone).map_err(|e| e.to_string())?;
        let opt_outs: Vec<StructuredValue<bool>> =
            serde_json::from_str(&self.opt_outs).map_err(|e| e.to_string())?;
        Ok(Guest {
            id,
            first_name,
            last_name,
            mail,
            phone,
            opt_outs,
        })
    }
}

/// Store SQLite pour les guests.
pub struct SqliteGuestStore {
    pub(super) pool: SqlitePool,
}

impl SqliteGuestStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
