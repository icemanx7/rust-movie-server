#![feature(plugin)]
#![plugin(rocket_codegen)]
mod readCsv;

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/id")]
fn get_movie() -> &'static str {
    "Hello, Movies!"
}

#[get("/num")]
fn get_number_movies() -> &'static str {
    "Hello, select number of movies!"
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/", routes![get_movie])
    .mount("/", routes![get_number_movies])
    .launch();
}
