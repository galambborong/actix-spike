use crate::user::User;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/users")]
async fn find_all() -> impl Responder {
  HttpResponse::Ok().json(vec![
    User {
      id: 1,
      email: "dummy-email@dummy.com".to_string(),
      name: "Dummy Mister".to_string(),
      // _age: 19,
    },
    User {
      id: 2,
      email: "bigger-dummy@dummy.com".to_string(),
      name: "Dummy Misssusss".to_string(),
      // _age: 21,
    },
  ])
}

#[get("/users/{id}")]
async fn find() -> impl Responder {
  HttpResponse::Ok().json(User {
    id: 1,
    email: "dummy-email@dummy.com".to_string(),
    name: "Dummy Mister".to_string(),
    // _age: 19,
  })
}

#[post("/users")]
async fn create(user: web::Json<User>) -> impl Responder {
  HttpResponse::Created().json(user.into_inner())
}

#[put("/users/{id}")]
async fn update(user: web::Json<User>) -> impl Responder {
  HttpResponse::Ok().json(user.into_inner())
}

#[delete("/users/{id}")]
async fn delete() -> impl Responder {
  HttpResponse::Ok().json(json!({"msg":"Successfully deleted"}))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(find_all);
  cfg.service(find);
  cfg.service(create);
  cfg.service(update);
  cfg.service(delete);
}
