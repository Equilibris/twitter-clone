pub mod ftquery;
pub mod make_model;
pub mod sanitizer;

use redis::FromRedisValue;
use rocket::serde::DeserializeOwned;
use serde::Serialize;
use serde_json::to_string;
use uuid::Uuid;

pub type ConType = redis::aio::MultiplexedConnection;

pub trait Idable {
    fn get_id(&self) -> Uuid;
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

pub fn get_con() -> ConType {
    // let client = crate::env::client::get();
    // let con = client.get_async_connection().await?;
    let con = crate::env::client::get_multiplexed_con();

    con.clone()
}

pub async fn write_con<Doc: Serialize + Idable>(
    doc: &Doc,
    con: &mut ConType
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
    let mut con = get_con();
    write_con(doc, &mut con).await
}

pub async fn exec_read_con<T: FromRedisValue>(
    cmd: &redis::Cmd,
    con: &mut ConType,
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
    con: &mut ConType,
) -> anyhow::Result<Option<Doc>> {
    let id = convert_uuid::<Doc>(id);
    let result = exec_read_con(redis::cmd("JSON.GET").arg(id), con).await?;

    Ok(result)
}
pub async fn read<Doc: redis::FromRedisValue>(id: &Uuid) -> anyhow::Result<Option<Doc>> {
    let mut con = get_con();

    read_con(id, &mut con).await
}

pub async fn bulk_read_con<Doc: DeserializeOwned + std::fmt::Debug>(
    ids: &Vec<Uuid>,
    con: &mut ConType,
) -> anyhow::Result<Vec<Option<Doc>>> {
    if ids.len() == 0 {
        return Ok(vec![]);
    }
    let mut cmd = &mut redis::cmd("JSON.MGET");

    for id in ids {
        let id = convert_uuid::<Doc>(id);
        cmd = cmd.arg(id);
    }

    cmd = cmd.arg("$");

    let result: Vec<String> = cmd.query_async(con).await?;

    let mut output = Vec::with_capacity(result.len());

    for i in result {
        let v: Vec<Doc> = serde_json::de::from_str(i.as_str())?;
        output.push(v.into_iter().next());
    }

    Ok(output)
}

pub async fn bulk_read<Doc: DeserializeOwned + std::fmt::Debug>(
    id: &Vec<Uuid>,
) -> anyhow::Result<Vec<Option<Doc>>> {
    let mut con = get_con();

    Ok(bulk_read_con(id, &mut con).await?)
}
