use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::{self, Idable};

use crate::make_model;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub uuid: Uuid,
    pub associate: Uuid,
    pub created_at: DateTime<Utc>,
}

impl Token {
    pub fn create_for_associate(associate: Uuid) -> Self {
        let uuid = Uuid::new_v4();
        let created_at = Utc::now();

        Self {
            uuid,
            associate,
            created_at,
        }
    }
    pub fn create_for<Doc: Idable>(doc: &Doc) -> Self {
        let associate = doc.get_id();

        Self::create_for_associate(associate)
    }

    pub fn should_renew(self: &Self) -> bool {
        let now = Utc::now();

        let delta = now - self.created_at;

        delta >= Duration::days(1)
    }

    pub async fn create_and_commit_con<Doc: Idable>(
        doc: &Doc,
        con: &mut redis::aio::Connection,
    ) -> anyhow::Result<Self> {
        let v = Self::create_for(doc);

        db::write_con(&v, con).await?;
        db::expire_con::<Self>(&v.uuid, con, Duration::days(3)).await?;

        Ok(v)
    }
    pub async fn create_and_commit<Doc: Idable>(doc: &Doc) -> anyhow::Result<Self> {
        let mut con = db::get_con().await?;

        Self::create_and_commit_con(doc, &mut con).await
    }
}

make_model!(Token);
