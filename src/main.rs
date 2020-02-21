use actix_web::{web, App, HttpResponse, middleware, HttpServer, Responder, Result};
use env_logger;

mod api;



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::DefaultHeaders::new()
                .header("Access-Control-Allow-Origin", "*")
            )
            .service(
                web::resource("/").route(
                    web::get().to(api::index)
                )
            )
            .service(
                web::resource("/login").route(
                    web::post().to(api::login)
                )
            )
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
