pub mod main;
pub mod not_found;
pub mod votecounter;

use actix_web::web;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(main::landing_page);
    cfg.service(votecounter::vote_counter);
    cfg.default_service(web::route().to(not_found::not_found));
}
