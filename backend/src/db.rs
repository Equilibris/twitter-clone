use chrono::Duration;
use serde::Serialize;
use serde_json::to_string;
use uuid::Uuid;

pub trait Idable {
    fn get_id(self: &Self) -> Uuid;
}

pub fn convert_uuid<T>(uuid: &Uuid) -> String {
    let mut buf = [b'\0'; 36];

    let id = uuid.to_simple().encode_lower(&mut buf);

    let format_val = id;
    let typename = std::any::type_name::<T>();
    format!("{}@{}", typename, format_val)
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
    let client = crate::env::client::get()?;
    let mut con = client.get_async_connection().await?;

    write_con(doc, &mut con).await
}

pub async fn read_con<Doc: redis::FromRedisValue>(
    id: &Uuid,
    con: &mut redis::aio::Connection,
) -> anyhow::Result<Option<Doc>> {
    let id = convert_uuid::<Doc>(id);
    let result = match redis::cmd("JSON.GET").arg(id).query_async(con).await {
        Ok(v) => Some(v),
        Err(e) => {
            let e: anyhow::Error = e.into();

            let x = e.to_string();
            let x = x.as_str();

            if let crate::models::make_model::VALUE_DOES_NOT_EXSIST_ERROR = x {
                None
            } else {
                return Err(e);
            }
        }
    };

    Ok(result)
}
pub async fn read<Doc: redis::FromRedisValue>(id: &Uuid) -> anyhow::Result<Option<Doc>> {
    let client = crate::env::client::get()?;
    let mut con = client.get_async_connection().await?;

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
    let client = crate::env::client::get()?;
    let mut con = client.get_async_connection().await?;

    expire_con::<Doc>(id, &mut con, duration).await
}
