use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

use crate::{api::token::Token, models::user::User};

pub struct TAU {
    pub token: Token,
    pub user: User,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for TAU {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match <Token as FromRequest<'_>>::from_request(request).await {
            rocket::outcome::Outcome::Success(v) => v,
            _ => return Outcome::Forward(()),
        };

        let user = match crate::db::read(&token.sub).await {
            Ok(Some(user)) => user,
            _ => return Outcome::Forward(()),
        };

        Outcome::Success(TAU { token, user })
    }
}
