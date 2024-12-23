use jsonwebtoken::{decode, encode, errors::Result as JwtResult, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;
use chrono::Duration;
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn create_jwt(user_id: Uuid) -> JwtResult<String> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    let expiration = chrono::Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Failed to set expiration time")
        .timestamp() as usize;

    let claims = Claims { sub: user_id, exp: expiration };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn decode_jwt(token: &str) -> JwtResult<Claims> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

pub fn generate_2fa_code() -> String {
    let mut rng = rand::thread_rng();
    (0..12).map(|_| rng.gen_range(0..10).to_string()).collect()
}

pub async fn validate_ota_code(email: &str, code: &str) -> Result<(), String> {
    let expected_code = "123456";

    if code == expected_code {
        Ok(())
    } else {
        Err("Invalid 2FA code".to_string())
    }
}
