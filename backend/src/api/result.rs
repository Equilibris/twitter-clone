use rocket::{http, response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResult<T: Serialize, E: Serialize> {
    #[serde(rename = "self")]
    pub me: String,
    pub data: Option<T>,
    pub error: Option<(u16, E)>,
}

impl<T: Serialize, E: Serialize> ApiResult<T, E> {
    pub fn data(me: String, data: T) -> Self {
        Self {
            me,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(me: String, status: u16, error: E) -> Self {
        Self {
            me,
            data: None,
            error: Some((status, error)),
        }
    }

    #[inline]
    pub fn status(self: &Self) -> u16 {
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
