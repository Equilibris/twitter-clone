pub mod ftquery;
pub mod make_model;

use chrono::Duration;
use redis::FromRedisValue;
use serde::Serialize;
use serde_json::to_string;
use uuid::Uuid;

pub trait Idable {
    fn get_id(self: &Self) -> Uuid;
}

pub fn create_prefix<T>() -> String {
    let typename = std::any::type_name::<T>();
    format!("{}@", typename)
}

pub fn convert_uuid<T>(uuid: &Uuid) -> String {
    let mut buf = [b'\0'; 36];

    let id = uuid.to_simple().encode_lower(&mut buf);

    let format_val = id;
    let prefix = create_prefix::<T>();
    format!("{}{}", prefix, format_val)
}

pub async fn get_con() -> anyhow::Result<redis::aio::Connection> {
    let client = crate::env::client::get()?;
    let con = client.get_async_connection().await?;
    Ok(con)
}

pub async fn write_con<Doc: Serialize + Idable>(
    doc: &Doc,
    con: &mut redis::aio::Connection,
) -> anyhow::Result<()> {
    let string = to_string(doc)?;

    let id = <Doc as Idable>::get_id(doc);
    let id = convert_uuid::<Doc>(&id);

    let result: redis::Value = redis::cmd("JSON.SET")
        .arg(id)
        .arg(".")
        .arg(string)
        .query_async(con)
        .await?;

    match result {
        redis::Value::Okay => Ok(()),
        _ => anyhow::bail!("failed to write to DB"),
    }
}
pub async fn write<Doc: Serialize + Idable>(doc: &Doc) -> anyhow::Result<()> {
    let mut con = get_con().await?;
    write_con(doc, &mut con).await
}

pub async fn exec_read_con<T: FromRedisValue>(
    cmd: &redis::Cmd,
    con: &mut redis::aio::Connection,
) -> anyhow::Result<Option<T>> {
    Ok(match cmd.query_async(con).await {
        Ok(v) => Some(v),
        Err(e) => {
            let e: anyhow::Error = e.into();

            let x = e.to_string();
            let x = x.as_str();

            if let self::make_model::VALUE_DOES_NOT_EXIST_ERROR = x {
                None
            } else {
                return Err(e);
            }
        }
    })
}
pub async fn read_con<Doc: FromRedisValue>(
    id: &Uuid,
    con: &mut redis::aio::Connection,
) -> anyhow::Result<Option<Doc>> {
    let id = convert_uuid::<Doc>(id);
    let result = exec_read_con(redis::cmd("JSON.GET").arg(id), con).await?;

    Ok(result)
}
pub async fn read<Doc: redis::FromRedisValue>(id: &Uuid) -> anyhow::Result<Option<Doc>> {
    let mut con = get_con().await?;

    read_con(id, &mut con).await
}

pub async fn expire_con<Doc: redis::FromRedisValue>(
    id: &Uuid,
    con: &mut redis::aio::Connection,
    duration: Duration,
) -> anyhow::Result<()> {
    let id = convert_uuid::<Doc>(id);
    let duration = duration.num_seconds();
    redis::cmd("EXPIRE")
        .arg(id)
        .arg(duration)
        .query_async(con)
        .await?;

    Ok(())
}

pub async fn expire<Doc: redis::FromRedisValue>(
    id: &Uuid,
    duration: Duration,
) -> anyhow::Result<()> {
    let mut con = get_con().await?;

    expire_con::<Doc>(id, &mut con, duration).await
}
