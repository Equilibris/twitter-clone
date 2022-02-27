use rocket::{serde::json::Json, Build, Rocket};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::result::ApiResult,
    db,
    models::user::{Me, PublicUser, User},
};

#[derive(Debug, Deserialize)]
struct SignUpData<'a> {
    pub name: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize)]
enum SignUpError {
    UserCreationError(&'static str),
    UserDbWriteError(&'static str),
    UserAlreadyExistsError(&'static str),
}

// fetch("/user/sign_up",{method:"POST",body:JSON.stringify({name:'hello',password:'world'})}).then(x=>x.json()).then(console.log)
#[post("/sign_up", data = "<data>")]
async fn sign_up(data: Json<SignUpData<'_>>) -> ApiResult<Me, SignUpError> {
    let url = "/user/sign_up".to_string();

    let user = match User::new(data.name.to_string(), data.password.to_string()) {
        Ok(x) => x,
        Err(_) => {
            return ApiResult::error(
                url,
                400,
                SignUpError::UserCreationError("Failed to create user"),
            )
        }
    };
    match db::write(&user).await {
        Ok(_) => (),
        Err(_) => {
            return ApiResult::error(
                url,
                400,
                SignUpError::UserDbWriteError("Failed to write user to db"),
            )
        }
    }

    ApiResult::data(url, Me::new(user))
}

#[derive(Debug, Serialize)]
enum GetUserError {
    UserDoesNotExist(Uuid),
    UnknownError(String),
}

// fetch("/user/3cc2d059-9098-4f1a-bab1-561f084561a3").then(x=>x.json()).then(console.log)
#[get("/<id>")]
async fn get_user<'a>(id: Uuid) -> ApiResult<PublicUser, GetUserError> {
    let url = format!("/user/{}", id);

    let id = id.into();

    let user = match db::read::<User>(&id).await {
        Ok(Some(usr)) => usr,
        Ok(None) => return ApiResult::error(url, 404, GetUserError::UserDoesNotExist(id)),
        Err(e) => {
            println!("Error occured from db::read: {}", e);

            let error = format!("Unknown error occured: {}", e);
            return ApiResult::error(url, 500, GetUserError::UnknownError(error));
        }
    };

    ApiResult::data(url, PublicUser::new(user))
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/user", routes![sign_up, get_user])
}
