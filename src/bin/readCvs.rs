extern crate csv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::process;

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    ID: i32,
    Name: String,
    Year: String,
}

fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn read_movies_from_file() -> Result<Vec<Movie>, Box<Error>> {
    let mut stack = Vec::new();
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
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

fn main() {
    println!("Hello, world!");
    let movies = read_movies_from_file();
    let json_data = match movies {
        Err(_) => None,
        Ok(data) => Some(return_movies_json(data)),
    };
    println!("{:?}", json_data);
}
