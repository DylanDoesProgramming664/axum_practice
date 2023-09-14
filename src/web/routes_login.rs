use crate::{web::AUTH_TOKEN, Error, Result};
use axum::{extract::Json, routing::post, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    return Router::new().route("/api/login", post(api_login));
}

async fn api_login(cookies: Cookies, Json(payload): Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic.
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real auth-token generation/signature.
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // Create Success Body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    return Ok(body);
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}