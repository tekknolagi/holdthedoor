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
use rocket_contrib::{Template, JSON};
use rocket::response::NamedFile;
use std::collections::HashMap;

mod util;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/static/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/db")]
fn get_db() -> JSON<util::Db> {
    let db = util::Db::open("db.json");
    JSON(db)
}

#[derive(FromForm)]
struct PersonForm { id: u64, name: String }
impl PersonForm {
    pub fn to_person(&self) -> util::Person {
        util::Person { id: self.id, name: self.name.clone() }
    }
}

#[post("/add", data="<user_input>")]
fn people_add(user_input: Form<PersonForm>) -> String {
    let input: util::Person = user_input.into_inner().to_person();
    let mut db = util::Db::open("db.json");
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
    let mut db = util::Db::open("db.json");
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

fn main() {
    rocket::ignite()
        .mount("/", routes![index, static_file, get_db])
        .mount("/people", routes![people_add, people_kill])
        .launch();
}
