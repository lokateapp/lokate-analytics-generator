use derive_more::{Display, Error};

use actix_web::{web, Error, Responder, ResponseError};
use pyo3::prelude::*;

pub async fn generate_purchase_analytics(path: web::Path<String>) -> Result<impl Responder, Error> {
    log::info!("generate purchase analytics endpoint is called");
    let user_id: String = path.into_inner();
    let code = include_str!("../../python/generate_purchase_analytics.py");

    match Python::with_gil(|py| -> Result<_, PyErr> {
        let activators = PyModule::from_code_bound(py, code, "", "")?;
        let categories_to_probabilities: Vec<(String, f64)> = activators
            .getattr("get_user_event_today")?
            .call1((user_id, ))?
            .extract()?;

        Ok(categories_to_probabilities)
    }) {
        Ok(probabilities_list) => Ok(web::Json(probabilities_list)),
        Err(e) => Err(PythonError {
            cause: e.to_string(),
        }.into()),
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Python error: {}", cause)]
struct PythonError {
    cause: String,
}

impl ResponseError for PythonError {}
