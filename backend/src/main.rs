#![feature(decl_macro, proc_macro_hygiene)]
#![feature(async_closure)]

mod api;
mod db;
mod env;
mod middleware;
mod models;
mod routes;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    unsafe {
        crate::env::client::generate()?;
        crate::env::pepper::generate();
    }

    crate::models::user::User::ensure_index().await?;
    crate::models::post::Post::ensure_index().await?;

    let r = rocket::build();
    let r = crate::routes::mount(r);

    r.launch().await?;

    Ok(())
}
