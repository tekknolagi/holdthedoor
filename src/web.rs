#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;

extern crate rocket;
extern crate serde_json;
extern crate rocket_contrib;

use rocket::request::Form;
use rocket_contrib::Template;
use std::collections::HashMap;

mod util;

#[get("/")]
fn index() -> String {
    let db = util::Db::open("db.json");
    format!("{:#?}", db.people[0])
}

#[get("/")]
fn rules_list() -> Template {
    let db = util::Db::open("db.json");
    Template::render("rules/list", &db)
}

#[get("/")]
fn people_list() -> Template {
    let db = util::Db::open("db.json");
    Template::render("people/list", &db)
}

#[get("/add")]
fn people_add() -> Template {
    let ctx = HashMap::<String,String>::new();
    Template::render("people/add", &ctx)
}

#[derive(FromForm)]
struct PersonForm { id: u64, name: String }
impl PersonForm {
    pub fn to_person(&self) -> util::Person {
        util::Person { id: self.id, name: self.name.clone() }
    }
}

#[post("/add", data="<user_input>")]
fn post_people_add(user_input: Form<PersonForm>) -> String {
    let input: util::Person = user_input.get().to_person();
    format!("{:#?}", input)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/rules", routes![rules_list])
        .mount("/people", routes![people_list, people_add, post_people_add])
        .launch();
}
