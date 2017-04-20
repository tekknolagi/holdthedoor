#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate chrono;

extern crate jfs;

// use self::chrono::prelude::Local;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod util;

#[derive(Serialize, Deserialize, Debug)]
struct Db {
    people: Vec<util::Person>,
    rules: Vec<util::RuleObj>,
}

fn main() {
    let path = Path::new("db.json");
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file
    };

    let mut s = String::new();
    let db : Db = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why.description()),
        Ok(_) => serde_json::from_str(&s).unwrap()
    };

    println!("Hello, {}", db.people[0].name);
}
