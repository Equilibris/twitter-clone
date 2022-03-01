use rocket::http::{Cookie, CookieJar, Status};
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

pub fn remove_user(jar: &CookieJar<'_>) {
    jar.remove(Cookie::new(USER_AUTH_COOKIE, ""));
}

pub fn write_user(token: Token, jar: &CookieJar<'_>) {
    let mut buf = [b'\0'; 36];

    let id = token.uuid.to_simple().encode_lower(&mut buf);

    let mut cookie = Cookie::new(USER_AUTH_COOKIE, id.to_string());
    cookie.set_http_only(true);
    // Expire after 3 days

    jar.add(cookie);
}

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

        if token.should_renew() {
            match Token::create_and_commit(&token).await {
                Ok(token) => write_user(token, cookies),
                _ => (),
            };
        }

        let user = db::read::<User>(&token.associate).await;

        match user {
            Ok(Some(user)) => Outcome::Success(user),
            _ => Outcome::Failure((Status::Unauthorized, AuthError::Invalid)),
        }
    }
}
