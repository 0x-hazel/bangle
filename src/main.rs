use axum::{debug_handler, extract::{Query, State}, response::{Html, IntoResponse, Redirect, Response}, routing::{get, post}, Form, Router};
use base64::prelude::{Engine, BASE64_URL_SAFE_NO_PAD as BASE64};
use rand::RngCore;
use sailfish::TemplateOnce;
use serde::Deserialize;
use shuttle_runtime::CustomError;
use sqlx::PgPool;
use tower_http::services::ServeDir;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Index;

#[derive(TemplateOnce)]
#[template(path = "editview.stpl")]
struct EditView {
    list: List,
    bangs: Vec<BangEntry>,
}

#[derive(TemplateOnce)]
#[template(path = "readview.stpl")]
struct ReadView {
    list: List,
    bangs: Vec<BangEntry>,
}

#[derive(TemplateOnce)]
#[template(path = "error.stpl")]
struct BangleError {
    err: &'static str,
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    #[serde(alias = "l")]
    list: i32,
    #[serde(alias = "k")]
    key: String,
}

#[derive(Deserialize)]
struct SearchQuery {
    #[serde(alias = "l")]
    list: i32,
    #[serde(alias = "k")]
    key: String,
    #[serde(alias = "q")]
    query: String,
}

#[derive(Debug, Deserialize)]
struct SetBase {
    list: i32,
    key: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct AddBang {
    list: i32,
    key: String,
    bang: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct DelBang {
    list: i32,
    key: String,
    bang: String,
}

type List = (i32, String, String, String);
type BangEntry = (String, String);

#[debug_handler]
async fn index(
    opts: Option<Query<ListQuery>>,
    State(state): State<ServerState>,
) -> Html<String> {
    match opts {
        None => Html(Index.render_once().expect("Unable to render index template")),
        Some(o) => {
            let list: Option<List> = sqlx::query_as("SELECT * FROM lists WHERE id = $1")
                .bind(o.list)
                .fetch_optional(&state.pool)
                .await
                .expect("Unable to query list");
            Html(match list {
                None => BangleError {err: "No such list exists"}.render_once(),
                Some(l) => {
                    let bangs: Vec<BangEntry> = sqlx::query_as("SELECT bang, link FROM bangs WHERE list_id = $1")
                        .bind(l.0)
                        .fetch_all(&state.pool)
                        .await
                        .expect("Unable to find bangs of list");
                    if o.key == l.2 {
                        EditView{list: l, bangs}.render_once()
                    } else {
                        ReadView{list: l, bangs}.render_once()
                    }
                }
            }.expect("Unable to render view template"))
        }
    }
}

#[debug_handler]
async fn search(
    opts: Option<Query<SearchQuery>>,
    State(state): State<ServerState>,
) -> Response {
    match opts {
        None => Html(BangleError {err: "Invalid options have been provided! You must provide a list (l), a key (k), and a query (q)"}.render_once().expect("Unable to render template")).into_response(),
        Some(opts) => {
            if opts.query == "!bangs" { return Redirect::to(&format!("/?l={}&k={}", opts.list, opts.key)).into_response() }
            let (bang, query) = match opts.query.chars().next() {
                Some('!') => {
                    let split = opts.query.split_once(' ').expect("Failed to split query string for bang");
                    (&split.0[1..], split.1)
                },
                _ => ("", opts.query.as_str())
            };
            println!("{}, {}", bang, query);
            let url: (String, Option<String>) = sqlx::query_as("SELECT lists.fallback, coalesce((SELECT bangs.link FROM bangs INNER JOIN lists ON lists.id = bangs.list_id WHERE bangs.list_id = $1 AND bangs.bang = $2 AND (lists.read_pw = $3 OR lists.edit_pw = $3)), null) FROM lists")
                .bind(opts.list)
                .bind(bang)
                .bind(&opts.key)
                .fetch_optional(&state.pool)
                .await
                .expect("Failed to query url of query")
                .expect("Unable to get data from url query");
            println!("{:?}", url);
            let url = match url.1 {
                None => url.0,
                Some(x) => x,
            };
            println!("{}", url);
            let url = url.replace("%s", query);
            println!("{}", url);
            println!("---");
            Redirect::to(&url).into_response()
        }
    }
}

#[debug_handler]
async fn create(
    State(state): State<ServerState>,
) -> Redirect {
    let edit = BASE64.encode(rand::thread_rng().next_u64().to_be_bytes());
    let mut read = BASE64.encode(rand::thread_rng().next_u64().to_be_bytes());
    while read == edit {
        read = BASE64.encode(rand::thread_rng().next_u64().to_be_bytes());
    }
    let id: (i32,) = sqlx::query_as("INSERT INTO lists (edit_pw, read_pw) VALUES ($1, $2) RETURNING id;")
        .bind(&edit)
        .bind(&read)
        .fetch_one(&state.pool)
        .await
        .expect("Unable to instantiate list");
    Redirect::to(&format!("/?l={}&k={}", id.0, edit))
}

#[debug_handler]
async fn add(
    State(state): State<ServerState>,
    Form(data): Form<AddBang>,
) {
    let list = sqlx::query("SELECT * FROM lists WHERE id = $1 AND edit_pw = $2")
        .bind(data.list)
        .bind(&data.key)
        .fetch_optional(&state.pool)
        .await
        .expect("Unable to query table to insert bang");
    if list.is_some() {
        sqlx::query("INSERT INTO bangs VALUES ($1, $2, $3)")
            .bind(data.list)
            .bind(data.bang)
            .bind(data.url)
            .bind(data.key)
            .execute(&state.pool)
            .await
            .expect("Unable to insert bang");
    }
}

#[debug_handler]
async fn del(
    State(state): State<ServerState>,
    Form(data): Form<DelBang>,
) {
    sqlx::query("DELETE FROM bangs USING lists WHERE bangs.list_id = $1 AND lists.id=bangs.list_id AND bangs.bang = $2 AND lists.edit_pw = $3")
        .bind(data.list)
        .bind(data.bang)
        .bind(data.key)
        .execute(&state.pool)
        .await
        .expect("Unable to delete bang");
}

#[debug_handler]
async fn set_base(
    State(state): State<ServerState>,
    Form(data): Form<SetBase>,
) {
    println!("{:?}", data);
    sqlx::query("UPDATE lists SET fallback = $1 WHERE id = $2 AND edit_pw = $3")
        .bind(data.url)
        .bind(data.list)
        .bind(data.key)
        .execute(&state.pool)
        .await
        .expect("Unable to update list fallback url");
}

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
        .route("/", get(index))
        .route("/!", get(search))
        .route("/create", post(create))
        .route("/add", post(add))
        .route("/del", post(del))
        .route("/base", post(set_base))
        .nest_service("/res", ServeDir::new("res"))
        .with_state(app_state);

    Ok(router.into())
}
