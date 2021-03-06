use std::collections::HashSet;

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::{PublicUser, User};
use crate::{
    db::{self, ftquery::FtQuery, ConType},
    guards::ppt::PPT,
    make_model,
};

mod comment_serde {
    pub static NO_VAL: &str = "0";

    pub fn serialize<S>(v: &Option<Uuid>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match v {
            Some(v) => {
                let v = format!("{}", v);

                serializer.serialize_str(v.as_str())
            }
            None => serializer.serialize_str(NO_VAL),
        }
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Option<Uuid>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(d.deserialize_str(OptionUuidVisitor)?)
    }
    struct OptionUuidVisitor;
    impl<'de> de::Visitor<'de> for OptionUuidVisitor {
        type Value = Option<Uuid>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a uuid or NO_VAL")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match uuid::Uuid::parse_str(v) {
                Ok(v) => Ok(Self::Value::Some(v)),
                Err(_) => Ok(Self::Value::None),
            }
        }
    }

    use std::fmt;

    use serde::{de, ser};
    use uuid::Uuid;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub uuid: Uuid,

    pub author: Uuid,
    pub message: String,

    #[serde(with = "self::comment_serde")]
    pub comment: Option<Uuid>,
    pub comment_count: usize,
    pub likes: HashSet<Uuid>,
    pub likes_count: usize,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct PublicPost {
    pub uuid: Uuid,

    pub author: crate::routes::user::GetUserResult,
    pub message: String,

    pub likes_count: usize,
    pub i_like: bool,

    pub comment_count: usize,
    pub comment: Option<Uuid>,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl PublicPost {
    pub async fn create(post: Post, ppt: &PPT) -> Self {
        Self {
            uuid: post.uuid,

            author: crate::routes::user::get_user(post.author).await,
            message: post.message,

            likes_count: post.likes_count,
            i_like: match &ppt.0 {
                Some(t) => post.likes.contains(&t.sub),
                None => false,
            },

            comment: post.comment,
            comment_count: post.comment_count,

            created_at: post.created_at,
        }
    }
    pub fn new_refed(post: Post, user: &User, ppt: &PPT) -> Self {
        Self {
            uuid: post.uuid,

            author: crate::api::result::ApiResult::data(
                format!("/user/{}", user.uuid),
                PublicUser::new_refed(user),
            ),
            message: post.message,

            likes_count: post.likes_count,
            i_like: match &ppt.0 {
                Some(t) => post.likes.contains(&t.sub),
                None => false,
            },

            comment: post.comment,
            comment_count: post.comment_count,

            created_at: post.created_at,
        }
    }
    pub fn new(post: Post, user: User, ppt: &PPT) -> Self {
        Self {
            uuid: post.uuid,

            author: crate::api::result::ApiResult::data(
                format!("/user/{}", user.uuid),
                PublicUser::new(user),
            ),
            message: post.message,

            likes_count: post.likes_count,
            i_like: match &ppt.0 {
                Some(t) => post.likes.contains(&t.sub),
                None => false,
            },

            comment: post.comment,
            comment_count: post.comment_count,

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

            comment: None,
            comment_count: 0,

            likes: HashSet::new(),
            likes_count: 0,

            created_at: Utc::now(),
        }
    }

    pub fn new_comment<T: crate::db::Idable>(message: String, author: &T, comment: Uuid) -> Self {
        Self {
            uuid: Uuid::new_v4(),

            author: author.get_id(),
            message,

            comment: Some(comment),
            comment_count: 0,

            likes: HashSet::new(),
            likes_count: 0,

            created_at: Utc::now(),
        }
    }

    // TODO: This should have a start date specified
    pub async fn query_feed_con(offset: usize, con: &mut ConType) -> anyhow::Result<FtQuery<Self>> {
        // Maybe do some maths to read backwards? Will this be more expensive maybe?
        Ok(redis::cmd("FT.SEARCH")
            .arg(POST_INDEX_NAME)
            .arg("@comments:{0}")
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
        let mut con = db::get_con();

        Ok(Self::query_feed_con(offset, &mut con).await?)
    }

    pub async fn query_author_feed_con(
        author: &Uuid,
        offset: usize,
        con: &mut ConType,
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
        let mut con = db::get_con();

        Ok(Self::query_author_feed_con(author, offset, &mut con).await?)
    }

    pub async fn query_comments_con(
        parent: &Uuid,
        offset: usize,
        con: &mut ConType,
    ) -> anyhow::Result<FtQuery<Self>> {
        let q = format!(
            "@comments:{{{}}}",
            db::sanitizer::sanitizer(parent.to_string().as_str())
        );

        Ok(redis::cmd("FT.SEARCH")
            .arg(POST_INDEX_NAME)
            .arg(q)
            .arg("SORTBY")
            .arg("feed")
            .arg("ASC")
            .arg("LIMIT")
            .arg(offset)
            .arg(25)
            .query_async(con)
            .await?)
    }
    pub async fn query_comments(parent: &Uuid, offset: usize) -> anyhow::Result<FtQuery<Self>> {
        let mut con = db::get_con();

        Ok(Self::query_comments_con(parent, offset, &mut con).await?)
    }
    // TODO: Stop-words and levenshtein distance functions..
    pub async fn search_con(
        term: &str,
        offset: usize,
        con: &mut ConType,
    ) -> anyhow::Result<FtQuery<Self>> {
        let q = db::sanitizer::sanitizer(term);

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
        let mut con = db::get_con();

        Ok(Self::search_con(term, offset, &mut con).await?)
    }

    pub async fn create_index_con(con: &mut ConType) -> anyhow::Result<()> {
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
            // Comment
            .arg("$.comment")
            .arg("AS")
            .arg("comments")
            .arg("TAG")
            .arg("SEPARATOR")
            .arg("@")
            // Likes
            .arg("$.likes_count")
            .arg("AS")
            .arg("likes")
            .arg("NUMERIC")
            .arg("SORTABLE")
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
        let mut con = db::get_con();

        if let Err(_) = redis::cmd("FT.INFO")
            .arg(POST_INDEX_NAME)
            .query_async::<ConType, ()>(&mut con)
            .await
        {
            println!("Index does not exist for posts, creating");
            Self::create_index_con(&mut con).await?
        }

        Ok(())
    }
}

make_model!(Post);
