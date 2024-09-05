use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod components;
mod routes;

const STYLE_CSS: &[u8] = include_bytes!("./static/output.css");
#[get("/style.css")]
async fn serve_css() -> impl Responder {
    HttpResponse::Ok().content_type("text/css").body(STYLE_CSS)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let address = (
        "0.0.0.0",
        std::env::var("PORT")
            .unwrap_or("3000".to_string())
            .parse::<u16>()
            .expect("env.PORT must be an integer"),
    );

    println!("Listening on {}:{}", address.0, address.1);

    HttpServer::new(|| App::new().service(serve_css).configure(routes::init))
        .bind(address)?
        .run()
        .await
}
