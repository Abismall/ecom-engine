use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData, errors::Result};
use chrono::{Utc, Duration};
use crate::services::identity::model::{Claims, UserRole};

const SECRET_KEY: &[u8] = b"your_secret_key";

pub fn create_token(user_id: &str, role: UserRole) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        role,
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
}

pub fn validate_token(token: &str) -> Result<TokenData<Claims>> {
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &Validation::default())
}
