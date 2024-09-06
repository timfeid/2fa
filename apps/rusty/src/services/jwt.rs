use std::ops::Add;

use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};

use rspc::Error;
use serde::{Deserialize, Serialize};

use crate::{
    error::{AppError, AppResult},
    models::user::User,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub jti: Option<String>,
    exp: u64,
}

pub struct JwtService {}

impl JwtService {
    pub fn decode(token: &str) -> AppResult<jsonwebtoken::TokenData<Claims>> {
        return Ok(decode(
            token,
            &DecodingKey::from_rsa_pem(include_bytes!("../../jwt.public.pem"))
                .map_err(|e| AppError::InteralServerError(e.to_string()))?,
            &Validation::new(Algorithm::RS256),
        )
        .map_err(|e| AppError::InteralServerError(e.to_string()))?);
    }

    pub fn create_for_user(user: &User, jti: Option<String>) -> AppResult<String> {
        let claims = Claims {
            jti,
            sub: user.get_id().to_string(),
            exp: get_current_timestamp().add(604800),
        };

        let response = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(include_bytes!("../../jwt.private.pem"))
                .map_err(|e| AppError::InteralServerError(e.to_string()))?,
        );

        Ok(response.map_err(|e| AppError::InteralServerError(e.to_string()))?)
    }

    // pub fn get_claims_from_cookies(cookies: Cookies) -> Option<TokenData<Claims>> {
    //     if let Some(cookie) = cookies.get("tits") {
    //         return JwtService::decode(cookie.value()).ok();
    //     }

    //     None
    // }

    // pub fn require_user_id_from_cookies(cookies: Cookies) -> Result<String, Error> {
    //     if let Some(token_data) = JwtService::get_claims_from_cookies(cookies) {
    //         return Ok(token_data.claims.sub);
    //     }

    //     Err(Error::new(
    //         rspc::ErrorCode::Unauthorized,
    //         "Log in first".to_owned(),
    //     ))
    // }

    // pub fn get_user_id_from_cookies(cookies: Cookies) -> Option<String> {
    //     if let Some(token_data) = JwtService::get_claims_from_cookies(cookies) {
    //         return Some(token_data.claims.sub);
    //     }

    //     None
    // }
}
