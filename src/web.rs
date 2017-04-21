#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate rocket;
extern crate serde_json;
extern crate rocket_contrib;

use rocket_contrib::Template;

mod util;

#[get("/")]
fn index() -> String {
    let db = util::Db::open("db.json");
    format!("{:#?}", db.people[0])
}

#[get("/newperson")]
fn newperson() -> Template {
    let context = util::Person { id: 5, name: "Butt".to_string() };
    Template::render("newperson", &context)
}


fn main() {
    rocket::ignite().mount("/", routes![index, newperson]).launch();
}
