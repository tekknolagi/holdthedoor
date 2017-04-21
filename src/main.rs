#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate chrono;

extern crate jfs;

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

impl Db {
    fn is_satisfied(&self, env: &util::Env) -> bool {
        self.rules.iter().any(|rule| {
            rule.is_satisfied(env)
        })
    }

    fn open(dbfile: &str) -> Db {
        let path = Path::new(dbfile);
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
            Ok(file) => file
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", path.display(), why.description()),
            Ok(_) => serde_json::from_str(&s).unwrap()
        }
    }
}

fn main() {
    let db = Db::open("db.json");
    let curenv = util::Env::current(db.people[0].clone());

    println!("Hello, {}", db.people[0].name);
    println!("satisfied? {:?}", db.is_satisfied(&curenv));
}
