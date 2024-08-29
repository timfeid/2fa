use std::sync::Arc;

use rspc::{Router, RouterBuilder};

use crate::Ctx;

pub fn create_account_router() -> rspc::RouterBuilder<Ctx> {
    Router::<Ctx>::new().query("test", |t| t(|ctx, input: ()| env!("CARGO_PKG_VERSION")))
}
