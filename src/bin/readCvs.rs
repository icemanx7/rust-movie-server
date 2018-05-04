extern crate csv;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::process;

#[derive(Debug, Deserialize)]
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

fn read_movies_from_file() -> Result<(), Box<Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record: Movie = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
     if let Err(err) = read_movies_from_file() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
