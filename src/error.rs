use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,
    TicketIdNotFound { id: u64 },
    NoAuthTokenCookie,
    BadAuthTokenFormat,
    CtxNotInReqExt,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        return write!(fmt, "{self:?}");
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        return response;
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        return match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            Self::TicketIdNotFound { .. } => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),
            Self::NoAuthTokenCookie => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        };
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
