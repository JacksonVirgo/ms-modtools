use actix_web::{get, HttpResponse, Responder};
use maud::html;

use crate::components::header::{generate_header, Header};

#[get("/votecounter")]
pub async fn vote_counter() -> impl Responder {
    let header: maud::PreEscaped<String> =
        generate_header(Header::new().title("Vote Counter - MafiaScum ModTools"));
    let markup = html! {
        (header)

        div."w-screen h-screen" {
            main."flex flex-col w-screen h-full items-center justify-center bg-base text-primary" {
                h1."text-5xl font-bold text-center" { "Vote Counter" }
                div."pt-2 flex flex-row gap-2" {
                    span."text-lg" {
                        "This is a placeholder for the vote counter."
                    }
                }
            }
        }
    };
    let html = markup.into_string();
    HttpResponse::Ok().body(html)
}
