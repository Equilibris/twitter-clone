#![feature(decl_macro, proc_macro_hygiene)]
#![feature(async_closure)]

use middleware::index_insurance;
use rocket::{figment::Figment, Build, Rocket};

use crate::middleware::cors;

mod api;
mod db;
mod env;
mod guards;
mod middleware;
mod models;
mod routes;

#[macro_use]
extern crate rocket;

fn construct_figment() -> Figment {
    Figment::from(rocket::Config::default())
        .merge(("log_level", rocket::config::LogLevel::Critical))
        .merge((
            rocket::Config::PORT,
            std::env::var("PORT")
                .unwrap_or("8000".to_string())
                .parse::<u16>()
                .unwrap(),
        ))
        .merge((rocket::Config::ADDRESS, "0.0.0.0"))
}

pub fn build_rocket() -> Rocket<Build> {
    let figment = construct_figment();

    let r = rocket::custom(figment)
        .attach(index_insurance::IndexInsurance)
        .attach(cors::CORS);
    let r = routes::mount(r);
    r
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let r = build_rocket();

    r.launch().await?;

    Ok(())
}
