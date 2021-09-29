use crate::constants::APPLICATION_JSON;
use actix_web::web::Path;
use actix_web::{get, post, HttpResponse};

#[get("/tweets")]
async fn get_tweets() -> HttpResponse {
    // TODO obtener todos los tweets
    let tweets = ["tweet 1: hola", "tweet 2: chao"];
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets)
}

#[post("/tweets")]
async fn create_tweets() -> HttpResponse {
    // TODO crear el tweet
    let nuevo_tweet = "Este es mi nuevo tweet";
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(nuevo_tweet)
}

#[get("/tweets/{id}")]
async fn get_tweet_by_id(path: Path<(String,)>) -> HttpResponse {
    // TODO obtener tweet con el id
    let tweet = format!("este es el tweet {:?}", path.0);
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweet)
}
