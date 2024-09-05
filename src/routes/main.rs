use actix_web::{get, HttpResponse, Responder};
use maud::html;

use crate::components::header::{generate_header, Header};

#[get("/")]
pub async fn landing_page() -> impl Responder {
    let header = generate_header(Header::new().title("MafiaScum ModTools"));
    let markup = html! {
        (header)

        div."w-screen h-screen" {
            main."flex flex-col w-screen h-full items-center justify-center bg-base text-primary" {
                h1."text-5xl font-bold text-center" { "MafiaScum ModTools" }
                h2."text-2xl text-center" {
                    span {"Assorted community-driven tools for " }
                    a."underline hover:pointer hover:no-underline" href="https://www.mafiascum.net" { "MafiaScum" }
                }
                div."pt-4 flex flex-row gap-2" {
                    a."py-2 px-4 bg-secondary text-primary hover:bg-primary hover:text-base rounded-lg" href="/votecounter" {
                        "Vote Counter"
                    }
                }
            }
        }
    };
    let html = markup.into_string();
    HttpResponse::Ok().body(html)
}
