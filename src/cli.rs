#[macro_use]
extern crate serde_derive;

mod util;

fn main() {
    let db = util::Db::open("db.json");
    let curenv = util::Env::current(db.people[0].clone());

    println!("Hello, {}", db.people[0].name);
    println!("satisfied? {:?}", db.is_satisfied(&curenv));
}
