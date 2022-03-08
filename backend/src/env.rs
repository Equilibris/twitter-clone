pub(crate) mod client {
    extern crate redis;

    static mut CLIENT_SINGLETON_OBJECT_REFERENCE: Option<redis::Client> = None;

    pub fn get() -> Result<&'static redis::Client, redis::RedisError> {
        unsafe {
            match CLIENT_SINGLETON_OBJECT_REFERENCE {
                Some(ref x) => Ok(x),
                None => generate(),
            }
        }
    }

    #[cfg(debug_assertions)]
    pub unsafe fn generate() -> Result<&'static redis::Client, redis::RedisError> {
        let redis_conn_url = "redis://127.0.0.1:6379";

        CLIENT_SINGLETON_OBJECT_REFERENCE = Some(redis::Client::open(redis_conn_url)?);

        match CLIENT_SINGLETON_OBJECT_REFERENCE {
            Some(ref x) => Ok(x),
            None => panic!("Unreachable"),
        }
    }

    #[cfg(not(debug_assertions))]
    pub unsafe fn generate() -> Result<&'static redis::Client, redis::RedisError> {
        let redis_url = std::env::var("REDIS_URL").expect("Missing env variable `REDIS_URL`");
        let redis_password =
            std::env::var("REDIS_PASSWORD").expect("Missing env variable `REDIS_PASSWORD`");

        let redis_conn_url = format!("redis://:{}@{}", redis_password, redis_url);

        CLIENT_SINGLETON_OBJECT_REFERENCE = Some(redis::Client::open(redis_conn_url)?);

        match CLIENT_SINGLETON_OBJECT_REFERENCE {
            Some(ref x) => Ok(x),
            None => panic!("Unreachable"),
        }
    }
}

pub(crate) mod pepper {
    pub type Pepper = [u8; 32];

    static mut PEPPER_SINGLETON_OBJECT_REFERENCE: Option<Pepper> = None;

    pub fn get() -> Pepper {
        unsafe {
            match PEPPER_SINGLETON_OBJECT_REFERENCE {
                Some(x) => x,
                None => generate(),
            }
        }
    }
    pub unsafe fn generate() -> Pepper {
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
        }

        PEPPER_SINGLETON_OBJECT_REFERENCE = Some(buffer);
        buffer
    }
}

pub(crate) mod jwt_secret {
    static mut JWT_SECRET_SINGLETON_OBJECT_REFERENCE: Option<String> = None;

    pub fn get() -> &'static [u8] {
        unsafe {
            match &JWT_SECRET_SINGLETON_OBJECT_REFERENCE {
                Some(x) => x.as_ref(),
                None => generate(),
            }
        }
    }
    pub unsafe fn generate() -> &'static [u8] {
        let v = std::env::var("JWT_SECRET");

        JWT_SECRET_SINGLETON_OBJECT_REFERENCE = Some(if cfg!(debug_assertions) {
            v.unwrap_or("no-secret".to_string())
        } else {
            v.expect("Missing env variable `JWT_SECRET`")
        });

        if let Some(x) = &JWT_SECRET_SINGLETON_OBJECT_REFERENCE {
            x.as_ref()
        } else {
            panic!("Unreachable")
        }
    }
}
