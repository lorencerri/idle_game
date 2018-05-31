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
static DATA_PATH: &'static str = "data/data.json";

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct User {
    name: String,
    balance: i64,
}

fn main() {

    // Create File Path
    let path = Path::new(DATA_PATH);
    let display = path.display();

    // Open File
    let mut file = File::open(&path);

    // Check if file was found
    if file.is_ok() {
        println!("File Found!");
    } else {
        println!("File NOT Found!");
    }

}
