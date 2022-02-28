pub fn failover_version_handler<T>(_stream: &[u8]) -> redis::RedisResult<T> {
    let typename = std::any::type_name::<T>();

    let error = format!("Model {} does not implement a version handeler", typename);

    redis::RedisResult::Err(redis::RedisError::from((
        redis::ErrorKind::TypeError,
        VERSIONING_ERROR,
        error,
    )))
}

pub const VERSIONING_ERROR: &str = "Versioning error.";
pub const VALUE_DOES_NOT_EXIST_ERROR: &str = "Value does not exsist.";

#[macro_export]
macro_rules! make_model {
    ($type:ident) => {
        use crate::db::make_model::failover_version_handler;

        make_model!($type, failover_version_handler);
    };
    ($type:ident, $version_handeler:ident) => {
        impl crate::db::Idable for $type {
            fn get_id(self: &Self) -> Uuid {
                self.uuid
            }
        }

        impl redis::FromRedisValue for $type {
            fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
                match v {
                    redis::Value::Data(data) => {
                        let result: Self = match serde_json::from_slice(data) {
                            Ok(v) => v,
                            Err(_) => return $version_handeler(data),
                        };
                        Ok(result)
                    }
                    redis::Value::Nil => Err(redis::RedisError::from((
                        redis::ErrorKind::TypeError,
                        crate::db::make_model::VALUE_DOES_NOT_EXIST_ERROR,
                    ))),
                    _ => unimplemented!("{:?}", v),
                }
            }
        }
    };
}
