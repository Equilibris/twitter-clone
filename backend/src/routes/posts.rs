use std::collections::HashMap;

use rocket::{serde::json::Json, Build, Rocket};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
struct CreatePostData {
    message: String,
}

#[derive(Serialize)]
enum PostError {
    UnknownError(String),
    PostDoesNotExist(String),
    BadPost,
}

#[post("/create", data = "<data>")]
async fn create(data: Json<CreatePostData>, tau: TAU) -> ApiResult<PublicPost, PostError> {
    let url = "/posts/create".to_string();

    if data.message.len() > 150 {
        return ApiResult::error(url, 400, PostError::BadPost);
    }

    let user = tau.user;

    let post = Post::new(data.message.to_owned(), &user);

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

// TODO: Basis value, where is starts using Date.now()
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
    let mut read_ids = Vec::with_capacity(feed.len());

    for i in feed.iter() {
        read_ids.push(i.author);
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

#[derive(Serialize)]
enum AuthorFeedError {
    AuthorDoesNotExist(&'static str),
    DbAccessError(String),
}

#[get("/<author>/<offset>")]
async fn author_feed(
    author: Uuid,
    offset: usize,
) -> ApiResult<Vec<ApiResult<PublicPost, ()>>, AuthorFeedError> {
    let url = format!("/post/{}/{}", author, offset);

    let feed = match Post::query_author_feed(&author, offset).await {
        Ok(feed) => feed.values(),
        Err(e) => {
            return ApiResult::error(
                url,
                500,
                AuthorFeedError::DbAccessError(format!("Failed to query for posts: {}", e)),
            );
        }
    };

    let author = match db::read::<User>(&author).await {
        Ok(Some(v)) => v,

        _ => {
            return ApiResult::error(
                url,
                404,
                AuthorFeedError::AuthorDoesNotExist("Author does not exist"),
            )
        }
    };

    let mut output = Vec::with_capacity(feed.len());

    for i in feed {
        output.push(ApiResult::data(
            format!("/post/{}", i.uuid),
            PublicPost::new_refed(i, &author),
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

    ApiResult::data(url, PublicPost::create(post).await)
}
pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/post", routes![create, get, feed, author_feed])
}
