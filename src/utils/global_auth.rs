use hmac::{Hmac, Mac};
use sha2::Sha256;

pub const JWT_AUTH_COOKIE_NAME: &str = "JWT";

pub fn get_jwt_signing_key() -> Hmac<Sha256> {
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap()
}

pub fn get_password_hash_secret() -> String {
    std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!")
}