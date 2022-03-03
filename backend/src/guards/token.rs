use rocket::{request::Outcome, Request};

use crate::api::token::Token;

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request
            .headers()
            .get_one("Authorization")
            .and_then(|x| {
                if x.starts_with("Basic ") {
                    Some(x.replace("Basic ", ""))
                } else {
                    None
                }
            })
            .and_then(|x| Token::decode(x).ok());

        match auth_header {
            Some(v) => Outcome::Success(v.claims),
            None => Outcome::Forward(()),
        }
    }
}
