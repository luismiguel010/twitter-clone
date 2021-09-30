#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
mod constants;
mod likes;
mod schema;
mod tweets;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no encontrada");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("No puede crear el pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(tweets::get_tweets)
            .service(tweets::create_tweets)
            .service(tweets::get_tweet_by_id)
            .service(likes::like_tweet)
            .service(likes::remove_like)
            .service(likes::get_likes_by_tweet)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
