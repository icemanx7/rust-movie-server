#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate csv;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::Json;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    ID: i32,
    Name: String,
    Year: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Movies {
    movies: Vec<Movie>,
}

impl Default for Movies {
    fn default() -> Movies {
        Movies { movies: Vec::new() }
    }
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

fn option_json_data() -> Option<Movies> {
    let movies_data = read_movies_from_file();
    let json_data = match movies_data {
        Err(_) => None,
        Ok(data) => Some(Movies { movies: data }),
    };
    return json_data;
}

#[get("/")]
fn index() -> Json<Movies> {
    let datas = option_json_data();
    let send_data = match datas {
        Some(json) => json,
        None => Movies::default(),
    };
    Json(send_data)
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
