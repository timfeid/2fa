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
        .query("list", |t| {
            t(|ctx, args: account::ListArgs| async move {
                Ok(account::AccountController::list(ctx, args).await?)
            })
        })
}
