pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    InteralServerError(String),
}

// impl From<AppError> for rspc::Error {
//     fn from(err: AppError) -> rspc::Error {
//         match err {
//             AppError::InteralServerError(s) => {
//                 rspc::Error::new(rspc::ErrorCode::InternalServerError, s)
//             }
//         }
//     }
// }
