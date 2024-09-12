use crate::{
    error::{AppError, AppResult},
    models::{
        account::{Account, AccountDetails, AccountDetailsWithCode},
        user::User,
    },
    services::jwt::JwtService,
    Ctx,
};
use bcrypt::verify;
use futures::future::try_join_all;
use rspc::{Router, RouterBuilder};
use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::{Pool, Postgres};
use totp_rs::{Algorithm, Secret, TOTP};

#[derive(Deserialize, Type)]
pub struct ListAccountArgs {}

#[derive(Serialize, Deserialize, Type)]
pub struct CreateAccountArgs {
    url: String,
    issuer: String,
    username: String,
}

pub struct AccountController {}
impl AccountController {
    pub async fn list(ctx: Ctx, args: ListAccountArgs) -> AppResult<Vec<AccountDetailsWithCode>> {
        let user = ctx.required_user()?;
        let accounts = Account::list_for_user(&ctx.pool, &user.sub).await?;
        let account_futures = accounts
            .iter()
            .map(|account| async move { account.into_response_with_code().await });

        let account_details_with_codes = try_join_all(account_futures).await?;

        Ok(account_details_with_codes)
    }

    pub async fn find(ctx: Ctx, id: String) -> AppResult<AccountDetailsWithCode> {
        let user = ctx.required_user()?;
        let accounts = Account::list_for_user(&ctx.pool, &user.sub).await?;
        let account = accounts
            .iter()
            .find(|account| account.id == id)
            .ok_or(AppError::BadRequest("Account not found".to_string()))?;

        Ok(account.into_response_with_code().await?)
    }

    pub async fn create(ctx: Ctx, args: CreateAccountArgs) -> AppResult<AccountDetailsWithCode> {
        let user = ctx.required_user()?;
        let account =
            Account::create_from_url(&ctx.pool, &user.sub, args.url, args.issuer, args.username)
                .await?;
        // let accounts = Account::list_for_user(&ctx.pool, &user.sub).await?;
        // let account = accounts
        //     .iter()
        //     .find(|account| account.id == id)
        //     .ok_or(AppError::BadRequest("Account not found".to_string()))?;

        Ok(account.into_response_with_code().await?)
    }

    pub async fn preview(ctx: Ctx, urls: Vec<String>) -> AppResult<Vec<CreateAccountArgs>> {
        let mut accounts: Vec<CreateAccountArgs> = vec![];
        for url in urls {
            if let Ok(details) = TOTP::from_url(&url) {
                if let Ok(account) = AccountDetails::try_from_totp(&details) {
                    accounts.push(CreateAccountArgs {
                        url,
                        issuer: details.issuer.unwrap_or_default(),
                        username: details.account_name,
                    });
                }
            }
        }

        Ok(accounts)
    }
}
