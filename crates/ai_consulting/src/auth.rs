use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData};
use crate::errors::ApiError;

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, ApiError> {
    let decoding_key = DecodingKey::from_secret(
        env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set")
            .as_bytes(),
    );
    decode::<Claims>(
        token,
        &decoding_key,
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| ApiError::Unauthorized("Invalid Token".into()))
}
