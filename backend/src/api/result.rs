use rocket::{http, response};
use serde::Serialize;

use crate::guards::ppt::PPT;

use super::token::Token;

#[derive(Debug, Serialize)]
pub struct ApiResult<T: Serialize, E: Serialize> {
    #[serde(rename = "self")]
    pub me: String,
    pub data: Option<T>,
    pub error: Option<(u16, E)>,
    pub refresh_token: Option<String>,
}

impl<T: Serialize, E: Serialize> ApiResult<T, E> {
    fn token_to_refresh_token(token: Token) -> Option<String> {
        if token.should_renew() {
            token.renew().encode().ok()
        } else {
            None
        }
    }

    pub fn data(me: String, data: T) -> Self {
        Self {
            me,
            data: Some(data),
            error: None,
            refresh_token: None,
        }
    }
    pub fn data_with_token(me: String, data: T, token: Token) -> Self {
        Self {
            me,
            data: Some(data),
            error: None,
            refresh_token: token.encode().ok(),
        }
    }
    pub fn data_with_refresh_token(me: String, data: T, token: Token) -> Self {
        Self {
            me,
            data: Some(data),
            error: None,
            refresh_token: Self::token_to_refresh_token(token),
        }
    }
    pub fn data_with_ppt(me: String, data: T, ppt: PPT) -> Self {
        match ppt.0 {
            Some(token) => Self::data_with_refresh_token(me, data, token),
            None => Self::data(me, data),
        }
    }

    pub fn error(me: String, status: u16, error: E) -> Self {
        Self {
            me,
            data: None,
            error: Some((status, error)),
            refresh_token: None,
        }
    }
    pub fn error_with_refresh_token(me: String, status: u16, error: E, token: Token) -> Self {
        Self {
            me,
            data: None,
            error: Some((status, error)),
            refresh_token: Self::token_to_refresh_token(token),
        }
    }
    pub fn error_with_ppt(me: String, status: u16, error: E, ppt: PPT) -> Self {
        match ppt.0 {
            Some(token) => Self::error_with_refresh_token(me, status, error, token),
            None => Self::error(me, status, error),
        }
    }

    #[inline]
    pub fn status(&self) -> u16 {
        if let Some((e, _)) = self.error {
            e
        } else {
            200
        }
    }
}

impl<'a, 'b: 'a, T: Serialize, E: Serialize> response::Responder<'a, 'b> for ApiResult<T, E> {
    fn respond_to(self, _request: &'a rocket::Request<'_>) -> response::Result<'b> {
        let status = self.status();

        match serde_json::to_string(&self) {
            Ok(x) => {
                let mut response = response::Response::new();

                response.set_header(http::ContentType::JSON);
                response.set_status(http::Status::new(status));
                response.set_sized_body(x.len(), std::io::Cursor::new(x));

                Ok(response)
            }
            Err(_e) => {
                Err(http::Status::InternalServerError)
                // let mut response = response::Response::new();

                // Err(response)
            }
        }
    }
}
