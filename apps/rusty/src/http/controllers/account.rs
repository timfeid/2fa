use bcrypt::verify;
use sqlx::{Pool, Postgres};

use rspc::{Router, RouterBuilder};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    error::{AppError, AppResult},
    models::{
        account::{Account, AccountDetails, AccountDetailsWithCode},
        user::User,
    },
    services::jwt::JwtService,
    Ctx,
};

#[derive(Deserialize, Type)]
pub struct ListArgs {}

pub struct AccountController {}
impl AccountController {
    pub async fn list(ctx: Ctx, args: ListArgs) -> AppResult<Vec<AccountDetails>> {
        let user = ctx.required_user()?;
        let accounts = Account::list_for_user(&ctx.pool, &user.sub).await?;

        Ok(accounts
            .iter()
            .map(|account| account.into_response())
            .collect())
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
}
