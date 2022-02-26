use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::password::Password;
use crate::make_model;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub uuid: Uuid,

    pub name: String,
    pub password_hash: Password,
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
    pub fn new(user: User) -> Self {
        Self {
            name: user.name,
            uuid: user.uuid,
        }
    }
}

impl User {
    pub fn new(
        name: String,
        password: String,
    ) -> Result<Self, std::array::TryFromSliceError> {
        Ok(Self {
            uuid: uuid::Uuid::new_v4(),
            name,
            password_hash: Password::new(password.as_str())?,
        })
    }
}

make_model!(User);
