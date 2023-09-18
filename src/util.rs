use serde::Serialize;
use sha2::Digest;

pub fn hash_string<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input);
    let hash = hasher.finalize().to_vec();
    hash
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub message: &'static str,
}

pub static ISO_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";
