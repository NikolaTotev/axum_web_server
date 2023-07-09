use axum::{response::IntoResponse, http::StatusCode};



pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error{
    LoginFail,

    //-- Auth errors.
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat, 
    AuthFailCtxNotInRequestExt,

    //-- Model errors.
    TicketDeleteFailIdNotFound {id: u64},
}

impl IntoResponse for Error
{
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12}- {self:?}", "INTO_RES");
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result<(), std::fmt> {
//         write!(fmt, "{self:?}")
//     }
// }