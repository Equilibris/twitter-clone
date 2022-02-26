use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::password::Password;
use crate::make_model;

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub uuid: Uuid,

    pub author: Uuid,
    pub message: String,

    pub comments: Vec<String>,
}

make_model!(Post);
