extern crate serde;
extern crate serde_json;
extern crate chrono;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fmt;

use self::chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct Person {
    pub id: u64,
    pub name: String,
}

pub struct Env {
    pub person: Person,
    pub date: DateTime<Local>,
}

impl Env {
    pub fn current(person: Person) -> Env {
        Env { person: person, date: Local::now() }
    }
}

trait Rule {
    fn is_satisfied(&self, env: &Env) -> bool;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateTimeRange {
    pub id: u64,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}

impl Rule for DateTimeRange {
    fn is_satisfied(&self, env: &Env) -> bool {
        env.person.id == self.id &&
        env.date >= self.start &&
        env.date <= self.end
    }
}

impl fmt::Display for DateTimeRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.start;
        let e = self.end;
        write!(f, "{} from {}/{} @ {}:{} to {}/{} @ {}:{}", self.id,
               s.month(), s.day(), s.hour(), s.minute(),
               e.month(), e.day(), e.hour(), e.minute())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TimeRange {
    day: u32,
    start: NaiveTime,
    end: NaiveTime,
}

impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} from {}:{} to {}:{}",
               vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"][self.day as usize],
               self.start.hour(), self.start.minute(),
               self.end.hour(), self.end.minute())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DayOfWeek {
    id: u64,
    days: Vec<TimeRange>,
}

impl fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} on ", self.id);
        for d in &self.days {
            write!(f, "{} ", d);
        }
        Ok(())
    }
}

impl Rule for DayOfWeek {
    fn is_satisfied(&self, env: &Env) -> bool {
        let current_day = env.date.weekday().number_from_monday();
        let current_time = env.date.time();
        env.person.id == self.id &&
            self.days.iter().any(|tr| {
                current_day == tr.day &&
                    current_time >= tr.start &&
                    current_time <= tr.end
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberOfSuite {
    id: u64,
}

impl fmt::Display for MemberOfSuite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} lives here", self.id)
    }
}

impl Rule for MemberOfSuite {
    fn is_satisfied(&self, env: &Env) -> bool {
        self.id == env.person.id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RuleObj {
    DTR(DateTimeRange),
    DOW(DayOfWeek),
    MOS(MemberOfSuite),
}

impl fmt::Display for RuleObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &RuleObj::DTR(ref a) => write!(f, "{}", a),
            &RuleObj::DOW(ref a) => write!(f, "{}", a),
            &RuleObj::MOS(ref a) => write!(f, "{}", a)
        }
    }
}

impl RuleObj {
    pub fn is_satisfied(&self, env: &Env) -> bool {
        match *self {
            RuleObj::DTR(ref rule) => rule.is_satisfied(env),
            RuleObj::DOW(ref rule) => rule.is_satisfied(env),
            RuleObj::MOS(ref rule) => rule.is_satisfied(env),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Db {
    pub people: Vec<Person>,
    pub rules: Vec<RuleObj>,
}

impl Db {
    pub fn is_satisfied(&self, env: &Env) -> bool {
        self.rules.iter().any(|rule| {
            rule.is_satisfied(env)
        })
    }

    pub fn open(dbfile: &str) -> Db {
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

    pub fn write(&self, dbfile: &str) -> serde_json::Result<()> {
        let json = serde_json::to_string(self)?;
        let s = format!("{}", json);
        let mut f = File::create(dbfile)?;
        f.write_all(s.as_bytes())?;
        f.sync_all()?;
        Ok(())
    }

    pub fn person_exists(&self, id: u64) -> bool {
        self.people.iter().any(|person| {
            person.id == id
        })
    }

    pub fn person_by_id(&self, id: u64) -> Option<&Person> {
        self.people.iter().find(|person| {
            person.id == id
        })
    }

    pub fn kill_person_by_id(&mut self, id: u64) -> () {
        self.people.retain(|person| {
            person.id != id
        })
    }
}


#[cfg(test)]
mod tests {
    extern crate chrono;
    extern crate time;
    use super::*;

    #[test]
    fn date_between() {
        let now = Local::now();
        let in_5_minutes = now + time::Duration::seconds(300);
        let in_3_minutes = now + time::Duration::seconds(180);
        let three_mins_ago = now - time::Duration::seconds(180);

        let rule = DateTimeRange {
            start: now, end: in_5_minutes
        };

        let person = Person { id: 5, name: "Max".to_string() };
        let env = Env { person: person.clone(), date: in_3_minutes };
        let badenv = Env { person: person.clone(), date: three_mins_ago };

        assert!(rule.is_satisfied(&env));
        assert!(!rule.is_satisfied(&badenv));
    }
}
