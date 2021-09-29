use actix_web::{App, HttpServer};

mod constants;
mod likes;
mod tweets;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
