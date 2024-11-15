mod properties;
mod responses;

use axum::{debug_handler, extract::{Query, State}, response::{Html, IntoResponse, Redirect, Response}, Form, Json};
use base64::prelude::{Engine, BASE64_URL_SAFE_NO_PAD as BASE64};
use properties::*;
use rand::RngCore;
use responses::*;
use sailfish::TemplateOnce;

use crate::{templates::{self, *}, ServerState};

/** Displays the main page, which varies based on provided arguments.
 * 
 * Displays the index page if no credentials are provided or the bang list if valid credentials are used.
 */
#[debug_handler]
pub async fn index(
    opts: Option<Query<ListQuery>>,
    State(state): State<ServerState>,
) -> Html<String> {
    match opts {
        None => Html(Index.render_once().expect("Unable to render index template")),
        Some(o) => {
            let list: Option<templates::List> = sqlx::query_as("SELECT * FROM lists WHERE id = $1 AND (read_pw = $2 OR edit_pw = $2)")
                .bind(o.list)
                .bind(&o.key)
                .fetch_optional(&state.pool)
                .await
                .expect("Unable to query list");
            Html(match list {
                None => BangleError {err: "Incorrect list number or key"}.render_once(),
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

/// Operates the search option using the list credentials and query provided.
#[debug_handler]
pub async fn search(
    opts: Option<Query<SearchQuery>>,
    State(state): State<ServerState>,
) -> Response {
    match opts {
        None => Html(BangleError {err: "Invalid options have been provided! You must provide a list (l), a key (k), and a query (q)"}.render_once().expect("Unable to render template")).into_response(),
        Some(opts) => {
            if opts.query == "!bangs" { return Redirect::to(&format!("/?l={}&k={}", opts.list, opts.key)).into_response() }
            let (bang, query) = match opts.query.chars().next() {
                Some('!') => {
                    let split = opts.query.split_once(' ');
                    match split {
                        None => (&opts.query.as_str()[1..], ""),
                        Some(split) => (&split.0[1..], split.1),
                    }
                },
                _ => ("", opts.query.as_str())
            };
            let url: (String, Option<String>) = sqlx::query_as(
                    "SELECT 
                    lists.fallback, 
                    coalesce((
                        SELECT bangs.link 
                        FROM bangs 
                        INNER JOIN lists 
                        ON lists.id = bangs.list_id 
                        WHERE bangs.list_id = $1 
                        AND bangs.bang = $2 
                        AND (lists.read_pw = $3 OR lists.edit_pw = $3
                    )), null) 
                    FROM lists"
                )
                .bind(opts.list)
                .bind(bang)
                .bind(&opts.key)
                .fetch_optional(&state.pool)
                .await
                .expect("Failed to query url of query")
                .expect("Unable to get data from url query");
            let (url, query) = match url.1 {
                None => (url.0, opts.query.as_str()),
                Some(x) => (x, query),
            };
            let url = url.replace("%s", query);
            Redirect::to(&url).into_response()
        }
    }
}

/// Creates a new bangle list and responds with a redirect to its edit page
#[debug_handler]
pub async fn create(
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

/// Adds a new bang to the given list
#[debug_handler]
pub async fn add(
    State(state): State<ServerState>,
    Form(data): Form<AddBang>,
) -> Json<AddResponse> {
    let list = sqlx::query("SELECT * FROM lists WHERE id = $1 AND edit_pw = $2")
        .bind(data.list)
        .bind(&data.key)
        .fetch_optional(&state.pool)
        .await
        .expect("Unable to query table to insert bang");
    if list.is_some() {
        if sqlx::query("INSERT INTO bangs VALUES ($1, $2, $3)")
            .bind(data.list)
            .bind(&data.bang)
            .bind(&data.url)
            .bind(&data.key)
            .execute(&state.pool)
            .await.is_ok() {
                return Json(AddResponse {
                    success: true,
                    details: Some(AddResponseDetails {
                        name: data.bang,
                        url: data.url,
                        id: data.list,
                        key: data.key
                    }),
                })
            }
    }
    Json(AddResponse {
        success: false,
        details: None,
    })
}

/// Removes a bang from the given list
#[debug_handler]
pub async fn del(
    State(state): State<ServerState>,
    Form(data): Form<DelBang>,
) -> Json<DeleteResponse> {
    match sqlx::query("DELETE FROM bangs USING lists WHERE bangs.list_id = $1 AND lists.id=bangs.list_id AND bangs.bang = $2 AND lists.edit_pw = $3")
        .bind(data.list)
        .bind(&data.bang)
        .bind(data.key)
        .execute(&state.pool)
        .await {
            Ok(_) => Json(DeleteResponse {
                success: true,
                bang: data.bang
            }),
            Err(_) => Json(DeleteResponse {
                success: false,
                bang: data.bang
            })
        }
}

#[debug_handler]
pub async fn edit(
    State(state): State<ServerState>,
    Form(data): Form<EditBang>
) -> Json<EditResponse> {
    match sqlx::query("UPDATE bangs SET link = $1 FROM lists WHERE bangs.list_id = $2 AND lists.id=bangs.list_id AND bangs.bang = $3 AND lists.edit_pw = $4")
        .bind(&data.url)
        .bind(data.list)
        .bind(&data.bang)
        .bind(data.key)
        .execute(&state.pool)
        .await {
            Ok(_) => Json(EditResponse {
                success: true,
                bang: data.bang,
                url: data.url
            }),
            Err(_) => Json(EditResponse {
                success: false,
                bang: data.bang,
                url: String::from("")
            })
        }
}

/// Updates the default search engine for the given list
#[debug_handler]
pub async fn set_base(
    State(state): State<ServerState>,
    Form(data): Form<SetBase>,
) -> Json<ChangeDefaultResponse> {
    match sqlx::query("UPDATE lists SET fallback = $1 WHERE id = $2 AND edit_pw = $3")
        .bind(&data.url)
        .bind(data.list)
        .bind(data.key)
        .execute(&state.pool)
        .await {
        Ok(_) => Json(ChangeDefaultResponse {
            success: true,
            current: data.url,
        }),
        Err(_) => Json(ChangeDefaultResponse {
            success: false,
            current: String::new()
        })
    }
}