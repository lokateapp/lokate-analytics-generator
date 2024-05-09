use actix_web::ResponseError;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "SQL error: {}", cause)]
pub struct SqlError {
    pub cause: String,
}

impl ResponseError for SqlError {}
