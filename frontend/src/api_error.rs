pub type ApiResult<T> = Result<T, ApiError>;

impl<T> From<ApiError> for ApiResult<T> {
    fn from(value: ApiError) -> Self {
        ApiResult::Err(value)
    }
}

#[derive(Debug)]
pub enum ApiError {
    Gloo(gloo::net::Error),
    InternalServerError,
}

impl From<gloo::net::Error> for ApiError {
    fn from(value: gloo::net::Error) -> Self {
        ApiError::Gloo(value)
    }
}