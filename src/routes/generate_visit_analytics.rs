use actix_web::{web, Error, Responder, ResponseError};
use chrono::{NaiveDate, Utc};
use derive_more::{Display, Error};
use serde::{Serialize, Serializer};
use sqlx::types::{chrono::DateTime, Uuid};
use sqlx::{prelude::FromRow, PgPool};
use std::collections::HashMap;

#[derive(Default, Serialize)]
struct SingleRoute(Vec<(f64, f64)>);

#[derive(Serialize)]
struct Routes(Vec<SingleRoute>);

impl From<Vec<Event>> for Routes {
    fn from(events: Vec<Event>) -> Self {
        let mut route_map: HashMap<(Uuid, NaiveDate), SingleRoute> = HashMap::new();
        for event in events {
            route_map
                .entry((event.customer_id, event.timestamp.date_naive()))
                .or_insert_with(SingleRoute::default)
                .0
                .push((event.x, event.y));
        }

        Routes(
            route_map
                .into_iter()
                .map(|((_customer_id, _date), route)| route)
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Serialize, FromRow)]
struct Event {
    customer_id: Uuid,
    #[serde(serialize_with = "serialize_dt")]
    timestamp: DateTime<Utc>,
    x: f64,
    y: f64,
}

pub async fn generate_visit_analytics(pool: web::Data<PgPool>) -> Result<impl Responder, Error> {
    log::info!("generate visit analytics endpoint is called");

    match sqlx::query_as::<_, Event>(
        r#"
        SELECT customer_id, enter_timestamp as timestamp, location_x as x, location_y as y 
        FROM events
        ORDER BY timestamp ASC
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(events) => {
            let routes: Routes = events.into();
            println!("{}", routes.0.len());
            Ok(web::Json(routes))
        }
        Err(e) => Err(SqlError {
            cause: e.to_string(),
        }
        .into()),
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "SQL error: {}", cause)]
struct SqlError {
    cause: String,
}

impl ResponseError for SqlError {}

fn serialize_dt<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    dt.format("%Y/%m/%d %H:%M:%S")
        .to_string()
        .serialize(serializer)
}
