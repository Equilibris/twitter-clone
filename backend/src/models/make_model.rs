#[macro_export]
macro_rules! make_model {
    ($type:ident) => {
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
                            Err(_) => panic!("Failed to hand of to serde"),
                        };
                        Ok(result)
                    }
                    _ => unimplemented!("{:?}", v),
                }
            }
        }
    };
}