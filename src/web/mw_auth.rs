use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, Request},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::{ctx::Ctx, models::ModelController, web::AUTH_TOKEN, Error, Result};

pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

    return Ok(next.run(req).await);
}

pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::NoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, exp, sign)) => Ok(Ctx::new(user_id)),
        Err(e) => Err(e),
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::NoAuthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    req.extensions_mut().insert(result_ctx);

    return Ok(next.run(req).await);
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        return parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::CtxNotInReqExt)?
            .clone();
    }
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) =
        regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(Error::BadAuthTokenFormat)?;

    return Ok((
        user_id
            .parse::<u64>()
            .map_err(|_| Error::BadAuthTokenFormat)?,
        exp.to_string(),
        sign.to_string(),
    ));
}
