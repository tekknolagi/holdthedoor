#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate rocket;
extern crate serde_json;
extern crate rocket_contrib;

use std::io;
use std::path::{Path, PathBuf};
use rocket::request::Form;
use rocket_contrib::JSON;
use rocket::response::NamedFile;
use std::collections::HashMap;
use std::hash::{Hash, SipHasher, Hasher};
use std::fmt;

mod util;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/static/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn opendb() -> util::Db {
    util::Db::open("db.json")
}

#[get("/db")]
fn get_db() -> JSON<util::Db> {
    let db = opendb();
    JSON(db)
}

#[post("/add", data="<user_input>")]
fn people_add(user_input: Form<util::Person>) -> String {
    let input: util::Person = user_input.into_inner();
    let mut db = opendb();
    if db.person_exists(input.id) {
        "Error: person exists".to_string()
    }
    else {
        db.people.push(input.clone());
        db.write("db.json");
        format!("{:#?}", input)
    }
}

#[post("/kill/<id>")]
fn people_kill(id: u64) -> String {
    let mut db = opendb();
    if db.person_exists(id) {
        let person = db.person_by_id(id).unwrap().clone();
        db.kill_person_by_id(id);
        db.write("db.json");
        format!("Success: killed {} (id {})", person.name, person.id)
    }
    else {
        "Error: person does not exist".to_string()
    }
}

struct Rules(Vec<util::RuleObj>);

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rules:\n");
        for r in &self.0 { write!(f, "{}\n", r); }
        Ok(())
    }
}


#[get("/")]
fn rules_list() -> String {
    let db = opendb();
    format!("{}", Rules(db.rules))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, static_file, get_db])
        .mount("/people", routes![people_add, people_kill])
        .mount("/rules", routes![rules_list])
        .launch();
}
