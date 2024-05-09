use actix_web::ResponseError;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "Python error: {}", cause)]
pub struct PythonError {
    pub cause: String,
}

impl ResponseError for PythonError {}
