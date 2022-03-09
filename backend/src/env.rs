pub(crate) mod client {
    extern crate redis;
    lazy_static::lazy_static! {
        static ref CLIENT_SINGLETON_OBJECT_REFERENCE: redis::Client = {
            let redis_conn_url = if cfg!(debug_assertions) { "redis://127.0.0.1:6379".to_string() } else {
                let redis_url = std::env::var("REDIS_URL").expect("Missing env variable `REDIS_URL`");
                let redis_password =
                    std::env::var("REDIS_PASSWORD").expect("Missing env variable `REDIS_PASSWORD`");

                 format!("redis://:{}@{}", redis_password, redis_url)
            };
            redis::Client::open(redis_conn_url).unwrap()
        };
    }

    pub fn get() -> &'static redis::Client {
        &CLIENT_SINGLETON_OBJECT_REFERENCE
    }
}

pub(crate) mod pepper {
    pub type Pepper = [u8; 32];

    lazy_static::lazy_static! {
        static ref PEPPER_SINGLETON_OBJECT_REFERENCE: Pepper = {
            let pepper = {
                let v = std::env::var("PEPPER");
                if cfg!(debug_assertions) {
                    v.unwrap_or("no-pepper".to_string())
                } else {
                    v.expect("Missing env variable `PEPPER`")
                }
            };
            let mut pepper = pepper.chars();

            let mut buffer = [0u8; 32];
            let mut index: usize = 0;

            while let Some(c) = pepper.next() {
                if index >= 32 {
                    break;
                }

                buffer[index] = c as u8;
                index += 1;
            };
            buffer
        };
    }

    pub fn get() -> Pepper {
        *PEPPER_SINGLETON_OBJECT_REFERENCE
    }
}

pub(crate) mod jwt_secret {
    lazy_static::lazy_static! {
        static ref JWT_SECRET_SINGLETON_OBJECT_REFERENCE: String = {
            let v = std::env::var("JWT_SECRET");

            if cfg!(debug_assertions) {
                v.unwrap_or("no-secret".to_string())
            } else {
                v.expect("Missing env variable `JWT_SECRET`")
            }
        };
    }

    pub fn get() -> &'static [u8] {
        JWT_SECRET_SINGLETON_OBJECT_REFERENCE.as_ref()
    }
}
