use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Info {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct MyObj {
    message: String,
    statusCode: i32,
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .body("Hey there!")
}

pub async fn login(info: web::Json<Info>) -> Result<HttpResponse>  {
    if info.username == "admin" && info.password == "123456" {
        Ok(
            HttpResponse::Ok()
                .json(MyObj {
                    statusCode:200,
                    message:"ok".to_string()
                })
        )
    } else {
        Ok(
            HttpResponse::Ok()
                .json((MyObj {
                    statusCode:201,
                    message:"fail".to_string()
                }))
        )
    }
}