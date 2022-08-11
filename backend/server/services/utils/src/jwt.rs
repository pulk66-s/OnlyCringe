use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}
pub fn create(data: &str) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", data);

    let token_str = claims.sign_with_key(&key).unwrap();
    return token_str;
}

pub fn verify(to_verify: &str, jwt: &str) -> bool {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let claims: BTreeMap<String, String> = jwt.verify_with_key(&key).unwrap();
    return claims["sub"] == to_verify;
}
