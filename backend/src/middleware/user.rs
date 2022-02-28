use rocket::http::Status;
use rocket::request::{Outcome, Request};
use uuid::Uuid;

use crate::db;
use crate::models::token::Token;
use crate::models::user::User;

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Invalid,
}

const USER_AUTH_COOKIE: &str = "user_auth_cookie";

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for User {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();

        let cookie = cookies
            .get(USER_AUTH_COOKIE)
            .and_then(|cookie| cookie.value().parse::<Uuid>().ok());

        let req = match cookie {
            Some(cookie) => db::read::<Token>(&cookie).await,
            _ => return Outcome::Failure((Status::Unauthorized, AuthError::Missing)),
        };
        let token = match req {
            Ok(Some(token)) => token,
            _ => return Outcome::Failure((Status::Unauthorized, AuthError::Invalid)),
        };

        // if token.should_renew() {
        //     let mut buf = [b'\0'; 36];

        //     let id = token.uuid.to_hyphenated().encode_lower(&mut buf);

        //     let cookie = Cookie::new(USER_AUTH_COOKIE, id.to_string()).http_only();
        // }

        let user = db::read::<User>(&token.associate).await;

        match user {
            Ok(Some(user)) => Outcome::Success(user),
            _ => Outcome::Failure((Status::Unauthorized, AuthError::Invalid)),
        }
    }
}
