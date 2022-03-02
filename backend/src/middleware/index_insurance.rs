use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Build, Request, Response, Rocket};

use crate::models::post::Post;
use crate::models::user::User;

pub struct IndexInsurance;

#[rocket::async_trait]
impl Fairing for IndexInsurance {
    fn info(&self) -> Info {
        Info {
            name: "Ensure index presence",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        match (User::ensure_index().await, Post::ensure_index().await) {
            (Ok(_), Ok(_)) => Ok(rocket),
            _ => Err(rocket),
        }
    }
}
