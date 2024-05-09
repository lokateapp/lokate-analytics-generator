use crate::errors::PythonError;
use actix_web::{web, Error, Responder};
use pyo3::prelude::*;

pub async fn generate_purchase_analytics(path: web::Path<String>) -> Result<impl Responder, Error> {
    log::info!("generate purchase analytics endpoint is called");
    let customer_id: String = path.into_inner();
    let code = include_str!("../../python/generate_purchase_analytics.py");

    match Python::with_gil(|py| -> Result<_, PyErr> {
        let activators = PyModule::from_code_bound(py, code, "", "")?;
        let mut categories_to_probabilities: Vec<(String, f64)> = activators
            .getattr("get_user_event_today")?
            .call1((customer_id,))?
            .extract()?;

        categories_to_probabilities
            .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let top_categories: Vec<String> = categories_to_probabilities
            .into_iter()
            .take(4)
            .map(|(category, _)| category)
            .collect();

        Ok(top_categories)
    }) {
        Ok(top_categories) => Ok(web::Json(top_categories)),
        Err(e) => Err(PythonError {
            cause: e.to_string(),
        }
        .into()),
    }
}
