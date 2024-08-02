use std::time::Duration;

use axum::http::StatusCode;
use chrono::Utc;
use dotenvy_macro::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims{
    exp:usize,
    iat:usize
}
pub fn create()-> Result<String, StatusCode>{
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::from_secs(30);
    now+=expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claims{exp, iat};
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claim, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}