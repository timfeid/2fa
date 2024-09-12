use std::sync::Arc;

use rspc::{Router, RouterBuilder};

use crate::{
    http::controllers::account::{self},
    Ctx,
};

pub fn create_account_router() -> rspc::RouterBuilder<Ctx> {
    Router::<Ctx>::new()
        .query("find", |t| {
            t(|ctx, id: String| async move { Ok(account::AccountController::find(ctx, id).await?) })
        })
        .query("preview", |t| {
            t(|ctx, args: Vec<String>| async move {
                Ok(account::AccountController::preview(ctx, args).await?)
            })
        })
        .mutation("create", |t| {
            t(|ctx, args: account::CreateAccountArgs| async move {
                Ok(account::AccountController::create(ctx, args).await?)
            })
        })
        .query("list", |t| {
            t(|ctx, args: account::ListAccountArgs| async move {
                Ok(account::AccountController::list(ctx, args).await?)
            })
        })
}
