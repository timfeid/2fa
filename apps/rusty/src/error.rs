pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    InteralServerError(String),
    BadRequest(String),
    Unauthorized,
}

impl From<AppError> for rspc::Error {
    fn from(err: AppError) -> rspc::Error {
        match err {
            AppError::InteralServerError(s) => {
                rspc::Error::new(rspc::ErrorCode::InternalServerError, s)
            }
            AppError::Unauthorized => {
                rspc::Error::new(rspc::ErrorCode::Unauthorized, "Unauthorized".to_owned())
            }
            AppError::BadRequest(s) => rspc::Error::new(rspc::ErrorCode::BadRequest, s),
        }
    }
}
