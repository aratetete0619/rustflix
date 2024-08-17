use actix_web::{web, App, HttpResponse, HttpServer};
mod lib;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = lib::establish_connection().await.expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
