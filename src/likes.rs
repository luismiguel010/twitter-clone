use crate::constants::APPLICATION_JSON;
use actix_web::web::Path;
use actix_web::{delete, get, post, HttpResponse};

#[get("/tweets/{id}/likes")]
async fn get_likes_by_tweet(path: Path<(String,)>) -> HttpResponse {
    let likes = 100;
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}

#[post("/tweets/{id}/likes")]
async fn like_tweet(path: Path<(String,)>) -> HttpResponse {
    // TODO hacer like en tweet
    let like = "ok";
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(like)
}

#[delete("/tweets/{id}/likes")]
async fn remove_like(path: Path<(String,)>) -> HttpResponse {
    // TODO disminuer el número de likes de ese tweet
    let like = "ok";
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
