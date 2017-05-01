#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate serde_derive;

extern crate rocket;

mod util;

fn main() {
    let db = util::Db::open("db.json");
    let curenv = util::Env::current(db.people[0].clone());

    println!("Hello, {}", db.people[0].name);
    println!("satisfied? {:?}", db.is_satisfied(&curenv));
}
