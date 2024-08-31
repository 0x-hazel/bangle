use axum::{routing::{get, post}, Router};
use shuttle_runtime::CustomError;
use sqlx::PgPool;
use tower_http::services::ServeDir;

mod routes;
mod templates;

#[derive(Clone)]
struct ServerState {
    pool: PgPool
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&pool).await.map_err(CustomError::new)?;
    // sqlx::query("DELETE FROM lists WHERE fallback = ''").execute(&pool).await.expect("Unable to purge invalid lists");
    let app_state = ServerState { pool };
    let router = Router::new()
        .route("/", get(routes::index))
        .route("/!", get(routes::search))
        .route("/create", post(routes::create))
        .route("/add", post(routes::add))
        .route("/del", post(routes::del))
        .route("/edit", post(routes::edit))
        .route("/base", post(routes::set_base))
        .nest_service("/res", ServeDir::new("res"))
        .with_state(app_state);

    Ok(router.into())
}
