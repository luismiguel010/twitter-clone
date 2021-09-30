use super::schema::tweets;
use crate::constants::APPLICATION_JSON;
use actix_web::web::{Data, Path};
use actix_web::{get, post, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::query_dsl::methods::{FilterDsl, LimitDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
struct Tweet {
    id: Uuid,
    created_at: NaiveDateTime,
    message: String,
}

impl Tweet {
    fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message,
        }
    }
}

#[get("/tweets")]
async fn get_tweets(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    use crate::schema::tweets::dsl::*;

    let conn = pool
        .get()
        .expect("No puede obtener conexión a la base de datos");
    let result = tweets.limit(10).load::<Tweet>(&conn); // select * from tweets order by created_at desc limit 10;
    let response = match result {
        Ok(tws) => tws,
        Err(_) => vec![],
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(response)
}

#[post("/tweets")]
async fn create_tweets(
    req_body: String,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    let nuevo_tweet = Tweet::new(req_body);
    let conn = pool
        .get()
        .expect("No se puede obtener conexión a la base de datos");

    diesel::insert_into(tweets::table)
        .values(&nuevo_tweet)
        .execute(&conn)
        .expect("Error al insertar tweet");

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(&nuevo_tweet)
}

#[get("/tweets/{id}")]
async fn get_tweet_by_id(
    path: Path<(String,)>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    use crate::schema::tweets::dsl::*;
    let conn = pool
        .get()
        .expect("No fue posible obtener conexión a la base de datos");
    let t_id = &path.0 .0;

    let t_id_uuid = Uuid::from_str(t_id);
    if t_id_uuid.is_err() {
        println!("Tweet id inválido, error {:?}", t_id_uuid.err());
        return HttpResponse::NotFound().await.unwrap();
    }

    let result = tweets
        .filter(id.eq(t_id_uuid.unwrap()))
        .load::<Tweet>(&conn);

    match result {
        Ok(rows) => match rows.first() {
            Some(tweet) => HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(tweet),
            _ => HttpResponse::NotFound().await.unwrap(),
        },
        Err(_) => HttpResponse::NotFound().await.unwrap(),
    }
}
