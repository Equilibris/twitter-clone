use rocket::{serde::json::Json, Build, Rocket};
use serde::{Deserialize, Serialize};

use crate::{
    api::result::ApiResult,
    db,
    models::{
        post::{Post, PublicPost},
        user::User,
    },
};

#[derive(Deserialize)]
struct CreatePostData<'a> {
    message: &'a str,
}

#[derive(Serialize)]
enum PostError {
    UnknownError(String),
    PostDoesNotExist(String),
}

#[post("/create", data = "<data>")]
async fn create(data: Json<CreatePostData<'_>>, user: User) -> ApiResult<PublicPost, PostError> {
    let message = data.message;
    let url = "/posts/create".to_string();

    let post = Post::new(message.to_string(), &user);

    match db::write(&post).await {
        Err(e) => ApiResult::error(
            url,
            500,
            PostError::UnknownError(format!("An unexpected error occurred: {}", e)),
        ),
        _ => ApiResult::data(url, PublicPost::create_from_user_and_post(post, user)),
    }
}

#[get("/feed/<offset>")]
async fn feed(offset: usize) -> ApiResult<Vec<ApiResult<PublicPost, ()>>, PostError> {
    let url = format!("/post/feed/{}", offset);

    let feed = match Post::query_feed(offset).await {
        Ok(feed) => feed,
        Err(e) => {
            return ApiResult::error(
                url,
                500,
                PostError::UnknownError(format!("An unexpected error occurred: {}", e)),
            )
        }
    };

    let feed = feed.values();

    let mut output = Vec::with_capacity(feed.len());
    for v in feed {
        output.push(ApiResult::data(
            format!("/post/{}", v.uuid),
            PublicPost::new(v).await,
        ))
    }

    ApiResult::data(url, output)
}

#[get("/<id>")]
async fn get(id: uuid::Uuid) -> ApiResult<PublicPost, PostError> {
    let url = format!("/post/{}", id);

    let post: Post = match db::read(&id).await {
        Ok(Some(v)) => v,
        _ => {
            return ApiResult::error(
                url,
                404,
                PostError::PostDoesNotExist(format!("Post with id {} does not exist", id)),
            )
        }
    };

    ApiResult::data(url, PublicPost::new(post).await)
}
pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/post", routes![create, get, feed])
}
