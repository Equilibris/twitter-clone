use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::password::Password;
use crate::{
    db::{self, ftquery::FtQuery, ConType},
    make_model,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub uuid: Uuid,

    pub name: String,
    pub password: Password,
}
#[derive(Debug, Serialize)]
pub struct Me {
    pub uuid: Uuid,

    pub name: String,
}
#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub uuid: Uuid,

    pub name: String,
}

impl Me {
    pub fn new(user: User) -> Self {
        Self {
            name: user.name,
            uuid: user.uuid,
        }
    }
}
impl PublicUser {
    pub fn new_refed(user: &User) -> Self {
        Self {
            name: user.name.clone(),
            uuid: user.uuid,
        }
    }
    pub fn new(user: User) -> Self {
        Self {
            name: user.name,
            uuid: user.uuid,
        }
    }
}

const USER_INDEX_NAME: &str = "user";

impl User {
    pub fn new(name: String, password: String) -> Result<Self, std::array::TryFromSliceError> {
        Ok(Self {
            uuid: uuid::Uuid::new_v4(),
            name,
            password: Password::new(password.as_str())?,
        })
    }

    // FIXME: This is more unstable than my mental state
    pub async fn query_username_con(
        name: &str,
        con: &mut crate::db::ConType,
    ) -> anyhow::Result<Option<Self>> {
        let name = db::sanitizer::sanitizer(name);
        let q = format!("@username:{{{}}}", name);
        let result: FtQuery<Self> = redis::cmd("FT.SEARCH")
            .arg(USER_INDEX_NAME)
            .arg(q)
            .query_async(con)
            .await?;

        let result = result.values().into_iter().next();

        Ok(result)
    }
    pub async fn query_username(name: &str) -> anyhow::Result<Option<Self>> {
        let mut con = db::get_con();

        Self::query_username_con(name, &mut con).await
    }

    pub async fn create_index_con(con: &mut ConType) -> anyhow::Result<()> {
        let _: () = redis::cmd("FT.CREATE")
            .arg(USER_INDEX_NAME)
            .arg("on")
            .arg("JSON")
            .arg("PREFIX")
            .arg("1")
            .arg(db::create_prefix::<Self>())
            .arg("SCHEMA")
            // Name search
            .arg("$.name")
            .arg("AS")
            .arg("name")
            .arg("TEXT")
            .arg("NOSTEM") 
            .arg("SORTABLE")
            // Username match
            .arg("$.name")
            .arg("AS")
            .arg("username")
            .arg("TAG")
            .arg("SEPARATOR")
            .arg("@")
            // Exec
            .query_async(con)
            .await?;

        Ok(())
    }

    pub async fn ensure_index() -> anyhow::Result<()> {
        let mut con = db::get_con();

        if let Err(_) = redis::cmd("FT.INFO")
            .arg(USER_INDEX_NAME)
            .query_async::<crate::db::ConType, ()>(&mut con)
            .await
        {
            println!("Index does not exist for user, creating");
            Self::create_index_con(&mut con).await?
        }

        Ok(())
    }
}

make_model!(User);
