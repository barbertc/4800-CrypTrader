#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use std::io;
use std::path::{Path, PathBuf};
use rocket::http::Method;
// use rocket_contrib::serve::StaticFiles;
use rocket::response::{NamedFile};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error, Cors, CorsOptions};

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000", // client
        "http://127.0.0.1:3000", // client
        "http://localhost:8000", // server
        "http://0.0.0.0:8000", // server
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Access-Control-Allow-Origin"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().expect("error while building CORS")
}

#[get("/api/myrocket")]
fn myrocket() -> String {
    "My rocket server".to_string()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("build/").join(file)).ok()
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    return NamedFile::open("build/index.html");
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, myrocket, files]).attach(make_cors())
}

fn main() {
    rocket().launch();
}

