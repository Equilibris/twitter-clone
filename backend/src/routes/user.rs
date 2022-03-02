use rocket::{http::CookieJar, serde::json::Json, Build, Rocket};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::result::ApiResult,
    db,
    middleware::user,
    models::{
        token::Token,
        user::{Me, PublicUser, User},
    },
};

#[derive(Debug, Deserialize)]
struct SignInAndUpData<'a> {
    pub name: &'a str,
    pub password: &'a str,
}

#[derive(Serialize)]
enum SignUpError {
    UserCreation(&'static str),
    UserDbWrite(&'static str),
    UserAlreadyExists(&'static str),
    FailedToCreateToken(&'static str),
    BadUsername(String),
}

fn username_is_valid(s: &str) -> Result<(), (bool, char)> {
    for char in s.chars() {
        if char.is_whitespace() {
            return Err((true, char));
        }
        if let '{' | '}' | '@' | '|' | ':' | '"' | '\'' = char {
            return Err((false, char));
        }
    }
    Ok(())
}

// fetch("/user/sign_up",{method:"POST",body:JSON.stringify({name:'hello',password:'world'})}).then(x=>x.json()).then(console.log)
#[post("/sign_up", data = "<data>")]
async fn sign_up(
    data: Json<SignInAndUpData<'_>>,
    jar: &CookieJar<'_>,
) -> ApiResult<Me, SignUpError> {
    let url = "/user/sign_up".to_string();
    let username = data.name;

    // Shitty input sanitization
    match username_is_valid(username) {
        Err((true, _)) => {
            return ApiResult::error(
                url,
                400,
                SignUpError::BadUsername("Username cannot include whitespace".to_string()),
            )
        }
        Err((false, c)) => {
            return ApiResult::error(
                url,
                400,
                SignUpError::BadUsername(format!("Username cannot include char: {}", c)),
            )
        }
        _ => (),
    }

    // OMG THIS PAIN
    match User::query_username(username).await {
        Ok(None) => (),
        _ => {
            return ApiResult::error(
                url,
                400,
                SignUpError::UserAlreadyExists("User already exists"),
            )
        }
    };

    // User creation
    let user = match User::new(username.to_string(), data.password.to_string()) {
        Ok(x) => x,
        Err(_) => {
            return ApiResult::error(url, 400, SignUpError::UserCreation("Failed to create user"))
        }
    };
    match db::write(&user).await {
        Ok(_) => (),
        Err(_) => {
            return ApiResult::error(
                url,
                400,
                SignUpError::UserDbWrite("Failed to write user to db"),
            )
        }
    }

    let token = match Token::create_and_commit(&user).await {
        Ok(token) => token,
        Err(_) => {
            return ApiResult::error(
                url,
                500,
                SignUpError::FailedToCreateToken("Failed to create token."),
            )
        }
    };

    user::write_user(token, jar);

    ApiResult::data(url, Me::new(user))
}

const USER_DOES_NOT_EXIST: &str = "user does not exist";

#[post("/sign_in", data = "<data>")]
async fn sign_in(
    data: Json<SignInAndUpData<'_>>,
    jar: &CookieJar<'_>,
) -> ApiResult<Me, &'static str> {
    let url = "/user/sign_in".to_string();

    let username = data.name;

    if let Err(_) = username_is_valid(username) {
        return ApiResult::error(url, 404, USER_DOES_NOT_EXIST);
    };

    let user = match User::query_username(username).await {
        Ok(Some(u)) => u,

        _ => return ApiResult::error(url, 404, USER_DOES_NOT_EXIST),
    };

    if user.password != data.password {
        return ApiResult::error(url, 404, USER_DOES_NOT_EXIST);
    }

    let token = Token::create_and_commit(&user).await.ok();
    if let Some(t) = token {
        user::write_user(t, jar);
    }

    ApiResult::data(url, Me::new(user))
}

#[post("/sign_out")] // FIXME: This leaks memory
async fn sign_out(jar: &CookieJar<'_>) -> ApiResult<(), ()> {
    #[allow(unused_must_use)]
    {
        user::remove_user(jar).await;
    }

    ApiResult::data("/user/sign_out".to_string(), ())
}

#[derive(Debug, Serialize)]
pub enum GetUserError {
    UserDoesNotExist(Uuid),
    UnknownError(String),
}

pub type GetUserResult = ApiResult<PublicUser, GetUserError>;
// fetch("/user/3cc2d059-9098-4f1a-bab1-561f084561a3").then(x=>x.json()).then(console.log)
#[get("/<id>")]
pub async fn get_user(id: Uuid) -> GetUserResult {
    let url = format!("/user/{}", id);

    let id = id.into();

    let user = match db::read::<User>(&id).await {
        Ok(Some(usr)) => usr,
        Ok(None) => return ApiResult::error(url, 404, GetUserError::UserDoesNotExist(id)),
        Err(e) => {
            println!("Error occurred from db::read: {}", e);

            let error = format!("Unknown error occured: {}", e);
            return ApiResult::error(url, 500, GetUserError::UnknownError(error));
        }
    };

    ApiResult::data(url, PublicUser::new(user))
}

#[derive(Serialize)]
struct MeError(&'static str);

#[get("/me")]
async fn me(user: User) -> ApiResult<Me, ()> {
    ApiResult::data("/user/me".to_string(), Me::new(user))
}
#[get("/me", rank = 2)]
async fn me_fail() -> ApiResult<(), MeError> {
    ApiResult::error("/user/me".to_string(), 401, MeError("Unauthenticated"))
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/user",
        routes![sign_up, get_user, me, me_fail, sign_in, sign_out],
    )
}
