#![feature(decl_macro, proc_macro_hygiene)]
#![feature(async_closure)]

use env::jwt_secret;
use middleware::index_insurance;

use crate::{
    env::{client, pepper},
    middleware::cors,
};

mod api;
mod db;
mod env;
mod guards;
mod middleware;
mod models;
mod routes;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    unsafe {
        client::generate()?;
        pepper::generate();
        jwt_secret::generate();
    }

    let r = rocket::build()
        .attach(index_insurance::IndexInsurance)
        .attach(cors::CORS);
    let r = routes::mount(r);

    r.launch().await?;

    Ok(())
}
