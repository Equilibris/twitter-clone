use rocket::{Build, Rocket};

use crate::api::result::ApiResult;

pub mod posts;
pub mod user;

#[catch(404)]
fn options_handler(req: &rocket::Request<'_>) -> ApiResult<(), ()> {
    let url = req.uri().to_string();

    if req.method() == rocket::http::Method::Options {
        ApiResult {
            me: url,
            data: None,
            error: None,
            refresh_token: None,
        }
    } else {
        ApiResult::error(url, 404, ())
    }
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    self::posts::mount(self::user::mount(
        rocket.register("/", catchers![options_handler]),
    ))
}
