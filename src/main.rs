extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

// Static Variables
static DATA_FILE: &'static str = "data/data.json";

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct User {
    name: String,
    balance: i64,
}

fn main() {

    println!("{}", DATA_FILE);

}
