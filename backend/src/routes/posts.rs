use std::collections::{HashMap, HashSet};

use rocket::{serde::json::Json, Build, Rocket};
use serde::{Deserialize, Serialize};

use crate::{
    api::result::ApiResult,
    db,
    guards::tau::TAU,
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
async fn create(data: Json<CreatePostData<'_>>, tau: TAU) -> ApiResult<PublicPost, PostError> {
    let message = data.message;
    let url = "/posts/create".to_string();

    let user = tau.user;

    let post = Post::new(message.to_string(), &user);

    match db::write(&post).await {
        Err(e) => ApiResult::error_with_refresh_token(
            url,
            500,
            PostError::UnknownError(format!("An unexpected error occurred: {}", e)),
            tau.token,
        ),
        _ => ApiResult::data_with_refresh_token(url, PublicPost::new(post, user), tau.token),
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
    let mut name_map = HashMap::with_capacity(feed.len());
    let mut set = HashSet::with_capacity(feed.len());
    let mut read_ids = Vec::with_capacity(feed.len());

    for i in feed.iter() {
        if !set.contains(&i.author) {
            read_ids.push(i.author);
            set.insert(i.author);
        }
    }
    for v in match db::bulk_read::<User>(&read_ids).await {
        Ok(x) => x,
        Err(e) => {
            return ApiResult::error(
                url,
                500,
                PostError::UnknownError(format!("Something went wrong during db search: {}", e)),
            )
        }
    } {
        if let Some(v) = v {
            name_map.insert(v.uuid, v);
        }
    }

    for v in feed {
        if let Some(author) = name_map.get(&v.author) {
            output.push(ApiResult::data(
                format!("/post/{}", v.uuid),
                PublicPost::new_refed(v, author),
            ))
        }
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

    ApiResult::data(url, PublicPost::create(post).await)
}
pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/post", routes![create, get, feed])
}
