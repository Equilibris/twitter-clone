// Potentially present token, Non failing token

use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

use crate::api::token::Token;

pub struct PPT(pub Option<Token>);

impl PPT {
    pub fn new(token: Token) -> Self {
        Self(Some(token))
    }
    pub fn no() -> Self {
        Self(None)
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for PPT {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        match <Token as FromRequest<'a>>::from_request(request).await {
            rocket::outcome::Outcome::Success(o) => Outcome::Success(Self::new(o)),
            _ => Outcome::Success(Self::no()),
        }
    }
}
