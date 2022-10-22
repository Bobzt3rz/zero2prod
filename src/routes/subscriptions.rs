use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct SubscriptionData {
    email: String,
    name: String,
}

#[post("/subscriptions")]
async fn subscribe(_info: web::Form<SubscriptionData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
