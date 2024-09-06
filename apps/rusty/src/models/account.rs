use rspc::selection;
use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::{query_as, Pool, Postgres};
use totp_rs::{Algorithm, Secret, TOTP};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "HashingAlgorithm")]
pub enum HashingAlgorithm {
    SHA1,
    SHA256,
    SHA512,
}

impl From<Algorithm> for HashingAlgorithm {
    fn from(algo: Algorithm) -> Self {
        match algo {
            Algorithm::SHA1 => HashingAlgorithm::SHA1,
            Algorithm::SHA256 => HashingAlgorithm::SHA256,
            Algorithm::SHA512 => HashingAlgorithm::SHA512,
        }
    }
}

impl From<HashingAlgorithm> for Algorithm {
    fn from(algo: HashingAlgorithm) -> Self {
        match algo {
            HashingAlgorithm::SHA1 => Algorithm::SHA1,
            HashingAlgorithm::SHA256 => Algorithm::SHA256,
            HashingAlgorithm::SHA512 => Algorithm::SHA512,
        }
    }
}

use crate::error::{AppError, AppResult};

use super::error::{ModelError, ModelResult};

pub struct Account {
    pub id: String,
    digits: i16,
    skew: i16,
    step: i64,
    secret: String,
    issuer: String,
    pub username: String,
    algorithm: HashingAlgorithm,
}

#[derive(Debug, Serialize, Type)]
pub struct AccountDetailsWithCode {
    pub id: String,
    pub issuer: String,
    pub username: String,
    pub code: String,
}

#[derive(Debug, Serialize, Type)]
pub struct AccountDetails {
    pub id: String,
    pub issuer: String,
    pub username: String,
}

impl Account {
    pub fn into_response(self: &Account) -> AccountDetails {
        AccountDetails {
            id: self.id.clone(),
            issuer: self.issuer.clone(),
            username: self.username.clone(),
        }
    }
    pub async fn list_for_user(
        pool: &Pool<Postgres>,
        user_id: &String,
    ) -> ModelResult<Vec<Account>> {
        query_as!(Account, "select id, digits, skew, step, secret, issuer, username, algorithm as \"algorithm: HashingAlgorithm\" from accounts where user_id = $1", user_id)
            .fetch_all(pool)
            .await
            .map_err(|e| ModelError::SqlError(e.to_string()))
    }

    pub async fn find(pool: &Pool<Postgres>, id: String) -> ModelResult<Account> {
        query_as!(Account, "select id, digits, skew, step, secret, issuer, username, algorithm as \"algorithm: HashingAlgorithm\" from accounts where id = $1", id)
            .fetch_one(pool)
            .await
            .map_err(|e| ModelError::SqlError(e.to_string()))
    }

    pub fn get_current_code(self: &Account) -> AppResult<TOTP> {
        TOTP::new(
            self.algorithm.clone().into(),
            self.digits as usize,
            self.skew as u8,
            self.step as u64,
            Secret::Raw(self.secret.as_bytes().to_vec())
                .to_bytes()
                .unwrap(),
            Some("test".to_string()),
            "dazed".to_string(),
        )
        .map_err(|e| AppError::InteralServerError(e.to_string()))
    }

    pub async fn into_response_with_code(&self) -> AppResult<AccountDetailsWithCode> {
        let code = self
            .get_current_code()
            .and_then(|c| Ok(c.generate_current().unwrap_or_default()))?;

        Ok(AccountDetailsWithCode {
            id: self.id.clone(),
            issuer: self.issuer.clone(),
            username: self.username.clone(),
            code,
        })
    }
}
