use std::time::SystemTime;

use anyhow::{anyhow, Result};
use axum::{
    headers::Cookie,
    http::{header, HeaderMap},
};
use serde::{Deserialize, Serialize};
/*use sha2::Digest;

pub fn hash_string<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input);
    let hash = hasher.finalize().to_vec();
    hash
}*/

#[derive(Serialize)]
pub struct GenericResponse {
    pub message: &'static str,
}

pub static ISO_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";
pub static JWT_TTL: u64 = 60 * 60 * 24 * 365;
pub static DEFAULT_COOKIE_OPTS: &str = "HttpOnly; Secure; path=/; SameSite=Lax";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: u64,
}

pub fn claims_from_cookie(cookie: Cookie) -> Result<Claims> {
    let jwt = cookie.get("jwt").ok_or(anyhow!("No JWT set"))?;
    let claims = jsonwebtoken::decode::<Claims>(
        &jwt,
        &jsonwebtoken::DecodingKey::from_secret(std::env::var("JWT_KEY").unwrap().as_bytes()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::default()),
    )?
    .claims;
    Ok(claims)
}

pub fn insert_jwt_into_headers(
    headers: &mut HeaderMap,
    sub: String,
    email: String,
    ttl: u64,
) -> () {
    let jwt = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            sub,
            email,
            exp: SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + ttl,
        },
        &jsonwebtoken::EncodingKey::from_secret(std::env::var("JWT_KEY").unwrap().as_bytes()),
    )
    .unwrap();

    headers.insert(
        header::SET_COOKIE,
        format!("jwt={jwt}; Max-Age={JWT_TTL}; {DEFAULT_COOKIE_OPTS}",)
            .parse()
            .unwrap(),
    );
}
