use actix_web::{get, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind(String::from("0.0.0.0:") + env!("BACKEND_PORT"))?
        .run()
        .await?;
    Ok(())
}

#[get("/bot-api")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello World";
    Ok(HttpResponse::Ok().body(response_body))
}
