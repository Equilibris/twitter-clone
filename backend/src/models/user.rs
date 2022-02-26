use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::password::Password;
use crate::make_model;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub uuid: Uuid,

    pub username: String,
    pub password_hash: Password,
}
#[derive(Debug, Serialize)]
pub struct Me {
    pub uuid: Uuid,

    pub username: String,
}
#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub uuid: Uuid,

    pub username: String,
}

impl Me {
    pub fn new(user: User) -> Self {
        Self {
            username: user.username,
            uuid: user.uuid,
        }
    }
}
impl PublicUser {
    pub fn new(user: User) -> Self {
        Self {
            username: user.username,
            uuid: user.uuid,
        }
    }
}

impl User {
    pub fn new(
        username: String,
        password: String,
    ) -> Result<Self, std::array::TryFromSliceError> {
        Ok(Self {
            uuid: uuid::Uuid::new_v4(),
            username,
            password_hash: Password::new(password.as_str())?,
        })
    }
}

make_model!(User);
