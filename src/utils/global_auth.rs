use hmac::{Hmac, Mac};
use sha2::Sha256;

pub const JWT_AUTH_COOKIE_NAME: &str = "JWT";
pub const JWT_AUTH_EXPIRATION_MINS: i64 = 10;

pub fn get_jwt_signing_key() -> Hmac<Sha256> {
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    Hmac::new_from_slice(jwt_secret.as_bytes()).expect("JWT_SECRET is not valid")
}

pub fn get_password_hash_secret() -> String {
    std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!")
}
