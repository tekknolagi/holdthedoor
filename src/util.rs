extern crate serde;
extern crate chrono;

use self::chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub id: u64,
    pub name: String,
}

pub struct Env {
    pub person: Person,
    pub date: DateTime<Local>,
}

trait Rule {
    fn is_satisfied(&self, env: &Env) -> bool;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateTimeRange {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}

impl Rule for DateTimeRange {
    fn is_satisfied(&self, env: &Env) -> bool {
        env.date >= self.start && env.date <= self.end
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeRange {
    day: u32,
    start: NaiveTime,
    end: NaiveTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DayOfWeek {
    days: Vec<TimeRange>,
}

impl Rule for DayOfWeek {
    fn is_satisfied(&self, env: &Env) -> bool {
        let current_day = env.date.weekday().number_from_monday();
        let current_time = env.date.time();
        self.days.iter().any(|tr| {
            current_day == tr.day &&
            current_time >= tr.start &&
            current_time <= tr.end
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RuleObj {
    DTR(DateTimeRange),
    DOW(DayOfWeek),
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
