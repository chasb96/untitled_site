use axum::http::StatusCode;
use log::error;

pub trait OrStatusCode {
    type Result;

    fn or_status_code(self, status_code: StatusCode) -> Self::Result;
}

impl<T, E> OrStatusCode for Result<T, E> 
{
    type Result = Result<T, StatusCode>;

    fn or_status_code(self, status_code: StatusCode) -> Self::Result {
        self.or(Err(status_code))
    }
}

impl<T> OrStatusCode for Option<T> 
{
    type Result = Result<T, StatusCode>;

    fn or_status_code(self, status_code: StatusCode) -> Self::Result {
        self.ok_or(status_code)
    }
}

pub trait OrInternalServerError {
    type Result;

    fn or_internal_server_error(self) -> Self::Result;
}

impl<T, E> OrInternalServerError for Result<T, E>
where
    Result<T, E>: OrStatusCode,
    E: std::fmt::Debug
{
    type Result = <Self as OrStatusCode>::Result;

    fn or_internal_server_error(self) -> Self::Result {
        self.inspect_err(|e| error!("{:?}", e)) // Force logging of 500's
            .or_status_code(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub trait OrNotFound {
    type Result;

    fn or_not_found(self) -> Self::Result;
}

impl<T> OrNotFound for T
where
    T: OrStatusCode,
{
    type Result = <T as OrStatusCode>::Result;

    fn or_not_found(self) -> Self::Result {
        self.or_status_code(StatusCode::NOT_FOUND)
    }
}

pub trait OrBadRequest {
    type Result;

    fn or_bad_request(self) -> Self::Result;
}

impl<T> OrBadRequest for T
where
    T: OrStatusCode
{
    type Result = <T as OrStatusCode>::Result;

    fn or_bad_request(self) -> Self::Result {
        self.or_status_code(StatusCode::BAD_REQUEST)
    }
}