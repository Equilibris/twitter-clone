use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::{PublicUser, User};
use crate::{
    db::{self, ftquery::FtQuery},
    make_model,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub uuid: Uuid,

    pub author: Uuid,
    pub message: String,

    pub comments: Vec<Uuid>,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct PublicPost {
    pub uuid: Uuid,

    pub author: crate::routes::user::GetUserResult,
    pub message: String,

    // pub comments: !,
    pub created_at: DateTime<Utc>,
}

impl PublicPost {
    pub async fn create(post: Post) -> Self {
        Self {
            uuid: post.uuid,

            author: crate::routes::user::get_user(post.author).await,
            message: post.message,

            created_at: post.created_at,
        }
    }
    pub fn new_refed(post: Post, user: &User) -> Self {
        Self {
            uuid: post.uuid,

            author: crate::api::result::ApiResult::data(
                format!("/user/{}", user.uuid),
                PublicUser::new_refed(user),
            ),
            message: post.message,

            created_at: post.created_at,
        }
    }
    pub fn new(post: Post, user: User) -> Self {
        Self {
            uuid: post.uuid,

            author: crate::api::result::ApiResult::data(
                format!("/user/{}", user.uuid),
                PublicUser::new(user),
            ),
            message: post.message,

            created_at: post.created_at,
        }
    }
}

const POST_INDEX_NAME: &'static str = "post";

impl Post {
    pub fn new<T: crate::db::Idable>(message: String, author: &T) -> Self {
        Self {
            uuid: Uuid::new_v4(),

            author: author.get_id(),
            message,

            comments: vec![],

            created_at: Utc::now(),
        }
    }

    // TODO: This should have a start date specified
    pub async fn query_feed_con(
        offset: usize,
        con: &mut redis::aio::Connection,
    ) -> anyhow::Result<FtQuery<Self>> {
        // Maybe do some maths to read backwards? Will this be more expensive maybe?
        Ok(redis::cmd("FT.SEARCH")
            .arg(POST_INDEX_NAME)
            .arg("*")
            .arg("SORTBY")
            .arg("feed")
            .arg("DESC")
            .arg("LIMIT")
            .arg(offset)
            .arg(25)
            .query_async(con)
            .await?)
    }
    pub async fn query_feed(offset: usize) -> anyhow::Result<FtQuery<Self>> {
        let mut con = db::get_con().await?;

        Ok(Self::query_feed_con(offset, &mut con).await?)
    }

    pub async fn query_author_feed_con(
        author: &Uuid,
        offset: usize,
        con: &mut redis::aio::Connection,
    ) -> anyhow::Result<FtQuery<Self>> {
        let q = format!(
            "@author:{{{}}}",
            db::sanitizer::sanitizer(author.to_string().as_str())
        );

        Ok(redis::cmd("FT.SEARCH")
            .arg(POST_INDEX_NAME)
            .arg(q)
            .arg("SORTBY")
            .arg("feed")
            .arg("DESC")
            .arg("LIMIT")
            .arg(offset)
            .arg(25)
            .query_async(con)
            .await?)
    }
    pub async fn query_author_feed(author: &Uuid, offset: usize) -> anyhow::Result<FtQuery<Self>> {
        let mut con = db::get_con().await?;

        Ok(Self::query_author_feed_con(author, offset, &mut con).await?)
    }

    // TODO: Stop-words and levenshtein distance functions..
    pub async fn search_con(
        term: &str,
        offset: usize,
        con: &mut redis::aio::Connection,
    ) -> anyhow::Result<FtQuery<Self>> {
        let q = db::sanitizer::sanitizer(term);

        println!("{}", q);

        Ok(redis::cmd("FT.SEARCH")
            .arg(POST_INDEX_NAME)
            .arg(q)
            .arg("LIMIT")
            .arg(offset)
            .arg(25)
            .query_async(con)
            .await?)
    }
    pub async fn search(term: &str, offset: usize) -> anyhow::Result<FtQuery<Self>> {
        let mut con = db::get_con().await?;

        Ok(Self::search_con(term, offset, &mut con).await?)
    }

    pub async fn create_index_con(con: &mut redis::aio::Connection) -> anyhow::Result<()> {
        let _: () = redis::cmd("FT.CREATE")
            .arg(POST_INDEX_NAME)
            .arg("on")
            .arg("JSON")
            .arg("PREFIX")
            .arg("1")
            .arg(db::create_prefix::<Self>())
            .arg("SCHEMA")
            // The feed
            .arg("$.created_at")
            .arg("AS")
            .arg("feed")
            .arg("NUMERIC")
            .arg("SORTABLE")
            // Search options
            .arg("$.message")
            .arg("AS")
            .arg("search")
            .arg("TEXT")
            // Author search
            .arg("$.author")
            .arg("AS")
            .arg("author")
            .arg("TAG")
            .arg("SEPARATOR")
            .arg("@")
            // Exec
            .query_async(con)
            .await?;

        Ok(())
    }

    pub async fn ensure_index() -> anyhow::Result<()> {
        let mut con = db::get_con().await?;

        if let Err(_) = redis::cmd("FT.INFO")
            .arg(POST_INDEX_NAME)
            .query_async::<redis::aio::Connection, ()>(&mut con)
            .await
        {
            println!("Index does not exist for posts, creating");
            Self::create_index_con(&mut con).await?
        }

        Ok(())
    }
}

make_model!(Post);
