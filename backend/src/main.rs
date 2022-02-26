#![feature(decl_macro, proc_macro_hygiene)]

mod api;
mod db;
mod env;
mod models;
mod routes;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    unsafe {
        crate::env::client::generate().unwrap();
        crate::env::pepper::generate();
    }

    let r = rocket::build();
    let r = crate::routes::mount(r);

    r
}
