#![feature(rustc_private)]
#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket_codegen;

extern crate serde;
extern crate dotenv;
extern crate rocket;
extern crate dim_database;
extern crate dim_scanners;

pub mod routes;
pub mod schema;
#[macro_use]
pub mod macros;
pub mod core;
pub mod tests;

fn main() {
    core::rocket().launch();
}
