use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use components::header::{generate_header, Header};
use dotenv::dotenv;
use maud::html;

mod components;

const STYLE_CSS: &[u8] = include_bytes!("./static/output.css");
#[get("/style.css")]
async fn serve_css() -> impl Responder {
    HttpResponse::Ok().content_type("text/css").body(STYLE_CSS)
}

#[get("/")]
async fn hello() -> impl Responder {
    let header = generate_header(Header::new());

    let html = html! {
        (header)
        h1."text-red-400 text-2xl" { "Hello, world!" }
    }
    .into_string();

    HttpResponse::Ok().body(html)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(serve_css)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(address)?
    .run()
    .await
}
