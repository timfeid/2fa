use std::{fs::write, future::IntoFuture, path::PathBuf, sync::Arc};

use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        request::Parts,
        Method,
    },
    routing::get,
};
use database::create_connection;
use error::{AppError, AppResult};
use http::routers::create_router;
use models::account::Account;
use services::jwt::{Claims, JwtService};
use sqlx::{Executor, Pool, Postgres};
use totp_rs::{Algorithm, Secret, TOTP};

use rspc::Router;
use tower_http::cors::{AllowOrigin, CorsLayer};

fn create_app(pool: Arc<Pool<Postgres>>) -> axum::Router {
    let router = create_router();
    let allowed_headers = [CONTENT_TYPE, AUTHORIZATION];
    let allowed_methods = [Method::GET, Method::POST, Method::OPTIONS];

    axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .nest(
            "/rspc",
            rspc_axum::endpoint(router, |parts: Parts| Ctx::new(pool, parts)),
        )
        .layer(
            CorsLayer::new()
                .allow_methods(allowed_methods)
                .allow_headers(allowed_headers)
                .allow_origin(AllowOrigin::mirror_request())
                .allow_credentials(true),
        )
}

mod database;
mod error;
mod http;
mod models;
mod services;

#[derive(Debug)]
pub struct Ctx {
    pub pool: Arc<Pool<Postgres>>,
    user: Option<Claims>,
}

impl Ctx {
    pub fn new(pool: Arc<Pool<Postgres>>, parts: Parts) -> Ctx {
        let user = match parts.headers.get("Authorization") {
            Some(bearer) => JwtService::decode(bearer.to_str().unwrap_or_default())
                .and_then(|r| Ok(r.claims))
                .ok(),
            None => None,
        };

        Ctx { pool, user }
    }

    pub fn required_user(self: &Ctx) -> AppResult<&Claims> {
        println!("{:?}", self);
        if self.user.is_none() {
            return Err(AppError::Unauthorized);
        }

        Ok(self.user.as_ref().unwrap())
    }
}

// async fn handler(context: Ctx) {
//     let account = Account::find(&context.pool.clone(), "test".to_string())
//         .await
//         .expect("hi");

//     let totp = &account.get_current_code().expect("hi");
//     let token = totp.generate_current().unwrap();

//     println!("{:?}, token: {}", account, token);
// }

#[tokio::main]
async fn main() {
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let pool = create_connection(&database_url).await;
    // handler(context).await;

    let app = create_app(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // let totp = TOTP::new(
    //     Algorithm::SHA1,
    //     6,
    //     1,
    //     30,
    //     Secret::Raw("TestSecretSuperSecret".as_bytes().to_vec())
    //         .to_bytes()
    //         .unwrap(),
    //     Some("test".to_string()),
    //     "dazed".to_string(),
    // )
    // .unwrap();
    // let token = totp.generate_current().unwrap();
    // println!("{}", token);
    // write("test.png", totp.get_qr_png().expect("unable to create png")).expect("Unable to write");
}
