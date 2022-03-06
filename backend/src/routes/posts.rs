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

async fn extract_name_map(values: &Vec<Post>) -> anyhow::Result<HashMap<Uuid, User>> {
    let mut name_map = HashMap::with_capacity(values.len());
    let mut read_ids = Vec::with_capacity(values.len());

    for i in values.iter() {
        read_ids.push(i.author);
    }

    for v in db::bulk_read::<User>(&read_ids).await? {
        if let Some(v) = v {
            name_map.insert(v.uuid, v);
        }
    }

    Ok(name_map)
}
fn transform_to_output(
    values: Vec<Post>,
    name_map: HashMap<Uuid, User>,
) -> Vec<ApiResult<PublicPost, ()>> {
    let mut output = Vec::with_capacity(values.len());

    for v in values {
        if let Some(author) = name_map.get(&v.author) {
            output.push(ApiResult::data(
                format!("/post/{}", v.uuid),
                PublicPost::new_refed(v, author),
            ))
        }
    }

    output
}

// TODO: Basis value, where is starts using Date.now()
#[get("/feed/<offset>")]
async fn feed(offset: usize) -> ApiResult<Vec<ApiResult<PublicPost, ()>>, PostError> {
    let url = format!("/post/feed/{}", offset);

    let feed = match Post::query_feed(offset).await {
        Ok(feed) => feed.values(),
        Err(e) => {
            return ApiResult::error(
                url,
                500,
                PostError::UnknownError(format!("An unexpected error occurred: {}", e)),
            )
        }
    };

    let output = match extract_name_map(&feed)
        .await
        .map(|name_map| transform_to_output(feed, name_map))
    {
        Ok(a) => a,
        Err(e) => return ApiResult::error(url, 500, PostError::UnknownError(e.to_string())),
    };

    ApiResult::data(url, output)
}

#[derive(Serialize)]
enum FeedError {
    AuthorDoesNotExist(&'static str),
    DbAccessError(String),
}

#[get("/<author>/<offset>")]
async fn author_feed(
    author: Uuid,
    offset: usize,
) -> ApiResult<Vec<ApiResult<PublicPost, ()>>, FeedError> {
    let url = format!("/post/{}/{}", author, offset);

    // Could be faster failing if other way around
    let feed = match Post::query_author_feed(&author, offset).await {
        Ok(feed) => feed.values(),
        Err(e) => {
            return ApiResult::error(
                url,
                500,
                FeedError::DbAccessError(format!("Failed to query for posts: {}", e)),
            );
        }
    };

    let author = match db::read::<User>(&author).await {
        Ok(Some(v)) => v,

        _ => {
            return ApiResult::error(
                url,
                404,
                FeedError::AuthorDoesNotExist("Author does not exist"),
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

#[get("/search/<term>/<offset>")]
async fn search(term: &str, offset: usize) -> ApiResult<Vec<ApiResult<PublicPost, ()>>, FeedError> {
    let url = format!("/post/search/{}/{}", term, offset);

    let feed = match Post::search(term, offset).await {
        Ok(v) => v.values(),
        Err(e) => {
            return ApiResult::error(
                url,
                500,
                FeedError::DbAccessError(format!("Failed to query for posts: {}", e)),
            );
        }
    };

    let output = match extract_name_map(&feed).await {
        Ok(name_map) => transform_to_output(feed, name_map),
        Err(e) => return ApiResult::error(url, 500, FeedError::DbAccessError(e.to_string())),
    };

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
    rocket.mount("/post", routes![create, get, feed, author_feed, search])
}
