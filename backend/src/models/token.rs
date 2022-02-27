use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::Idable;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    uuid: Uuid,
    associate: Uuid,
    created_at: DateTime<Utc>,
}

impl Token {
    pub fn create_for<Doc: Idable>(doc: Doc) -> Self {
        let associate = doc.get_id();
        let uuid = Uuid::new_v4();
        let created_at = Utc::now();

        Self {
            uuid,
            associate,
            created_at,
        }
    }

    pub fn should_renew(self: &Self) -> bool {
        let now = Utc::now();

        let delta = now - self.created_at;

        delta >= Duration::days(1)
    }

	
}
