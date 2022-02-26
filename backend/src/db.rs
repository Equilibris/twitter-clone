use serde::Serialize;
use serde_json::to_string;
use uuid::Uuid;

pub trait Idable {
    fn get_id(self: &Self) -> Uuid;
}

fn convert_uuid<T>(uuid: &Uuid) -> String {
    let mut buf = [b'\0'; 36];

    let id = uuid.to_simple().encode_lower(&mut buf);

    let format_val = id;
    let typename = std::any::type_name::<T>();
    format!("{}@{}", typename, format_val)
}

pub fn write<Doc: Serialize + Idable>(doc: &Doc) -> anyhow::Result<()> {
    let client = crate::env::client::get()?;
    let mut con = client.get_connection()?;

    let string = to_string(doc)?;

    let id = <Doc as Idable>::get_id(doc);
    let id = convert_uuid::<Doc>(&id);

    let result: redis::Value = redis::cmd("JSON.SET")
        .arg(id)
        .arg(".")
        .arg(string)
        .query(&mut con)?;

    match result {
        redis::Value::Okay => Ok(()),
        _ => anyhow::bail!("failed to write to DB"),
    }
}

pub fn read<Doc: redis::FromRedisValue>(id: &Uuid) -> anyhow::Result<Doc> {
    let client = crate::env::client::get()?;
    let mut con = client.get_connection()?;

    let id = convert_uuid::<Doc>(id);
    let result = redis::cmd("JSON.GET").arg(id).query(&mut con)?;

    Ok(result)
}
