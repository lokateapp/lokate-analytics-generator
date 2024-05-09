use crate::errors::{PythonError, SqlError};
use actix_web::{web, Error, Responder};
use chrono::{NaiveDate, Utc};
use pyo3::prelude::*;
use serde::{Serialize, Serializer};
use sqlx::types::{chrono::DateTime, Uuid};
use sqlx::{prelude::FromRow, PgPool};

#[derive(Debug, Serialize, FromRow)]
struct Event {
    beacon_id: Uuid,
    #[serde(serialize_with = "serialize_dt")]
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, FromRow)]
struct BeaconName(String);

async fn get_beacon_name(beacon_id: &Uuid, pool: &web::Data<PgPool>) -> Result<String, Error> {
    match sqlx::query_as::<_, BeaconName>(
        r#"
        SELECT name
        FROM beacons
        WHERE id = $1
        "#,
    )
    .bind(beacon_id)
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(beacon_name) => Ok(beacon_name.0),
        Err(e) => Err(SqlError {
            cause: e.to_string(),
        }
        .into()),
    }
}

async fn get_current_route(
    customer_id: &Uuid,
    pool: &web::Data<PgPool>,
) -> Result<Vec<String>, Error> {
    let today: NaiveDate = Utc::now().date_naive();
    match sqlx::query_as::<_, Event>(
        r#"
        SELECT beacon_id, enter_timestamp as timestamp
        FROM events
        WHERE customer_id = $1 AND DATE(enter_timestamp) = $2
        ORDER BY timestamp ASC
        "#,
    )
    .bind(customer_id)
    .bind(today)
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(events) => {
            let mut current_route = Vec::new();
            for event in events {
                current_route.push(get_beacon_name(&event.beacon_id, pool).await?);
            }
            Ok(current_route)
        }
        Err(e) => Err(SqlError {
            cause: e.to_string(),
        }
        .into()),
    }
}

pub async fn generate_visit_analytics(
    path: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    log::info!("generate visit analytics endpoint is called");
    let customer_id = Uuid::parse_str(&path.into_inner()).unwrap();
    let current_route = get_current_route(&customer_id, &pool).await?;
    let code = include_str!("../../python/generate_visit_analytics.py");

    match Python::with_gil(|py| -> Result<_, PyErr> {
        let activators = PyModule::from_code_bound(py, code, "", "")?;
        let next_route: String = activators
            .getattr("get_next_stop")?
            .call1((current_route,))?
            .extract()?;

        Ok(next_route)
    }) {
        Ok(next_route) => Ok(web::Json(next_route)),
        Err(e) => Err(PythonError {
            cause: e.to_string(),
        }
        .into()),
    }
}

fn serialize_dt<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    dt.format("%Y/%m/%d %H:%M:%S")
        .to_string()
        .serialize(serializer)
}
