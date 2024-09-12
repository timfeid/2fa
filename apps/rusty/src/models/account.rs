use rspc::selection;
use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::{query, query_as, Pool, Postgres};
use totp_rs::{Algorithm, Secret, TOTP};
use ulid::Ulid;

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "hashing_algorithm")]
pub enum HashingAlgorithm {
    SHA1,
    SHA256,
    SHA512,
}

impl From<String> for HashingAlgorithm {
    fn from(value: String) -> Self {
        match value.as_str() {
            "SHA1" => Self::SHA1,
            "SHA256" => Self::SHA256,
            "SHA512" => Self::SHA512,
            _ => Self::SHA1,
        }
    }
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
    pub next_step: String,
    pub step: String,
}

impl AccountDetailsWithCode {
    pub fn try_from_totp(details: TOTP) -> AppResult<AccountDetailsWithCode> {
        Ok(AccountDetailsWithCode {
            id: details.account_name.clone(),
            issuer: details.issuer.clone().unwrap_or("Uknown".to_string()),
            username: details.account_name.clone(),
            code: details
                .generate_current()
                .map_err(|_| AppError::InteralServerError("Something went wrong?".to_string()))?,
            next_step: details.account_name.clone(),
            step: details.account_name.clone(),
        })
    }
}

#[derive(Debug, Serialize, Type)]
pub struct AccountDetails {
    pub id: String,
    pub issuer: String,
    pub username: String,
}

impl AccountDetails {
    pub fn try_from_totp(details: &TOTP) -> AppResult<AccountDetails> {
        Ok(AccountDetails {
            id: details.account_name.clone(),
            issuer: details.issuer.clone().unwrap_or("Uknown".to_string()),
            username: details.account_name.clone(),
        })
    }
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
        let secret = Secret::Encoded(self.secret.clone());
        TOTP::new(
            self.algorithm.clone().into(),
            self.digits as usize,
            self.skew as u8,
            self.step as u64,
            secret.to_bytes().unwrap(),
            Some(self.issuer.clone()),
            self.username.clone(),
        )
        .map_err(|e| AppError::InteralServerError(e.to_string()))
    }

    pub async fn into_response_with_code(&self) -> AppResult<AccountDetailsWithCode> {
        let totp = self.get_current_code()?;
        let next_step = totp.next_step_current().unwrap_or_default().to_string();
        let code = totp.generate_current().unwrap_or_default();

        Ok(AccountDetailsWithCode {
            id: self.id.clone(),
            issuer: self.issuer.clone(),
            username: self.username.clone(),
            code,
            next_step,
            step: self.step.clone().to_string(),
        })
    }

    pub async fn create_from_url(
        pool: &Pool<Postgres>,
        sub: &str,
        url: String,
        issuer: String,
        username: String,
    ) -> AppResult<Account> {
        let details =
            TOTP::from_url(url).map_err(|e| AppError::InteralServerError(e.to_string()))?;
        let id = Ulid::new().to_string();

        query!(
            "insert into accounts
            (id, user_id, digits, skew, step, secret, issuer, username, algorithm)
            values
            ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            id,
            sub,
            details.digits.clone() as i16,
            details.skew.clone() as i16,
            details.step.clone() as i64,
            Secret::Raw(details.secret).to_encoded().to_string(),
            issuer,
            username,
            HashingAlgorithm::from(details.algorithm) as _
        )
        .execute(pool)
        .await
        .map_err(|e| ModelError::SqlError(e.to_string()))?;

        Ok(Account::find(pool, id).await?)
    }
}
