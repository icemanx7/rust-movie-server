#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate csv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    ID: i32,
    Name: String,
    Year: String,
}

fn read_movies_from_file() -> Result<Vec<Movie>, Box<Error>> {
    let mut stack = Vec::new();
    let file = File::open("./data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: Movie = result?;
        stack.push(record);
    }
    Ok(stack)
}

fn return_movies_json(movies: Vec<Movie>) -> Result<std::string::String, serde_json::Error> {
    let dr: Result<std::string::String, serde_json::Error> = movies
        .iter()
        .map(|ref y| serde_json::to_string(&y))
        .collect();
    return dr;
}

fn option_json_data() -> Option<Result<std::string::String, serde_json::Error>>{
    let movies = read_movies_from_file();
    let json_data = match movies {
        Err(_) => None,
        Ok(data) => Some(return_movies_json(data)),
    };
    return json_data;
}

#[get("/")]
fn index() -> &'static str {
    option_json_data();
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
