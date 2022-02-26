use std::fmt::Display;
use std::str::FromStr;

use rand::Rng;
use serde::{de::Visitor, Deserialize, Serialize};
use sha2::Digest;
use sha2::Sha256;

pub type Salt = [u8; 16];
pub type PasswordDigest = [u8; 32];

#[derive(Debug)]
pub struct Password {
    pub salt: Salt,
    pub digest: PasswordDigest,
}

struct PasswordHashVisitor;

impl<'de> Visitor<'de> for PasswordHashVisitor {
    type Value = Password;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A Password string, [8u; 16] + [8u; 32]")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Password::from_str(v) {
            Ok(v) => Ok(v),
            Err(e) => Err(E::custom(e)),
        }
    }
}

#[derive(Debug)]
pub enum PasswordParsingError {
    InvalidStringLength,
    StringNotFormattedCorrectly,

    DecodeError1(base64::DecodeError),
    DecodeError2(base64::DecodeError, base64::DecodeError),
}
impl Display for PasswordParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Password {
    pub fn new(s: &str) -> Result<Self, std::array::TryFromSliceError> {
        let mut hasher = Sha256::new();
        let mut rand_state = rand::thread_rng();

        let salt: Salt = rand_state.gen();
        let pepper = crate::env::pepper::get();

        hasher.update(pepper);
        hasher.update(salt);
        hasher.update(s);

        let digest = hasher.finalize().as_slice().try_into()?;

        Ok(Self { salt, digest })
    }
    pub fn comp(&self, s: &str) -> Result<bool, std::array::TryFromSliceError> {
        let mut hasher = Sha256::new();

        let pepper = crate::env::pepper::get();

        hasher.update(pepper);
        hasher.update(self.salt);
        hasher.update(s);

        let digest: PasswordDigest = hasher.finalize().as_slice().try_into()?;

        Ok({
            let mut time_safe_result = true;
            for (a, b) in digest.iter().zip(self.digest.iter()) {
                if a != b {
                    time_safe_result = false;
                }
            }
            time_safe_result
        })
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.salt == other.salt && self.digest == other.digest
    }
}
impl PartialEq<&str> for Password {
    fn eq(&self, other: &&str) -> bool {
        self.comp(*other).unwrap()
    }

    fn ne(&self, other: &&str) -> bool {
        !self.comp(*other).unwrap()
    }
}

impl Serialize for Password {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let encoded_salt = base64::encode_config(self.salt, base64::STANDARD);
        let encoded_digest = base64::encode_config(self.digest, base64::STANDARD);

        let v = format!("{}@{}", encoded_salt, encoded_digest);

        serializer.serialize_str(v.as_str())
    }
}
impl<'de> Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PasswordHashVisitor)
    }
}
impl std::str::FromStr for Password {
    type Err = PasswordParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('@') {
            Some((a, b)) => {
                let mut salt: Salt = [0u8; 16];
                let mut digest: PasswordDigest = [0u8; 32];
                match (
                    base64::decode_config_slice(a, base64::STANDARD, &mut salt),
                    base64::decode_config_slice(b, base64::STANDARD, &mut digest),
                ) {
                    (Ok(16), Ok(32)) => Ok(Self { salt, digest }),
                    (Ok(_), Err(a)) => Err(Self::Err::DecodeError1(a)),
                    (Err(a), Ok(_)) => Err(Self::Err::DecodeError1(a)),
                    (Err(a), Err(b)) => Err(Self::Err::DecodeError2(a, b)),
                    _ => Err(Self::Err::InvalidStringLength),
                }
            }
            None => Err(Self::Err::StringNotFormattedCorrectly),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    #[test]
    fn it_hashes() {
        let psw_string = "hello world";
        let other_psw_hash = Password::new(psw_string).unwrap();
        let psw_hash = Password::new(psw_string).unwrap();

        assert_ne!(psw_hash, other_psw_hash); // there is a 1/2^128 chance that this fails, that would piss me off

        println!("hashes: {:?}", other_psw_hash);
        println!("hashes: {:?}", psw_hash);
    }

    #[test]
    fn it_comps() {
        let other_psw_string = "Hello world";
        let psw_string = "hello world";
        let psw_hash = Password::new(psw_string).unwrap();

        assert_eq!(psw_hash, psw_string);
        assert_ne!(psw_hash, other_psw_string);

        println!("hashes: {:?}", psw_hash);
    }

    #[test]
    fn it_ser_and_des() {
        use serde_json::{from_str, to_string};

        let psw_string = "hello world";
        let psw_hash = Password::new(psw_string).unwrap();

        let s = to_string(&psw_hash).unwrap();
        println!("ser: {}", s);

        let d: Password = from_str(s.as_str()).unwrap();

        assert_eq!(psw_hash, d);

        assert_eq!(d, psw_string);
        assert_eq!(psw_hash, psw_string);
    }
}
