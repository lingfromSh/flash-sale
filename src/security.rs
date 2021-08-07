use sha2::Sha384;
use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey};
use std::collections::BTreeMap;
use mongodb::bson::oid::ObjectId;

const JWT_SECRET_KEY: &[u8; 64] = b"4JHul3lqoOJI1WB6tDAAApxJJwr0hr96IEje7WwJ1lNgj957LP3pSGJQeDYH8x+w";

pub fn generate_jwt_token(id: &ObjectId, username: &String) -> String {
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_SECRET_KEY).expect("Something wrong.");
    let mut claims = BTreeMap::new();
    claims.insert("id", id.to_string());
    claims.insert("username", username.to_string());
    claims.sign_with_key(&key).expect("Sign failed.")
}

pub fn verify_jwt_token(token: &str) -> BTreeMap<String, String> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(JWT_SECRET_KEY).expect("Something wrong.");
    let claims: BTreeMap<String, String> = token.verify_with_key(&key).expect("Verify failed.");
    claims
}