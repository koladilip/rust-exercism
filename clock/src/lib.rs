#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    relative_mins: i32,
}

static MINUTES_IN_DAY: i32 = 24 * 60;

fn compute_relative_mins(minutes: i32) -> i32 {
    (MINUTES_IN_DAY + minutes % MINUTES_IN_DAY) % MINUTES_IN_DAY
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let relative_mins = compute_relative_mins(hours * 60 + minutes);
        Clock {
            relative_mins: relative_mins
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let relative_mins = compute_relative_mins(self.relative_mins + minutes);
        Clock {
            relative_mins: relative_mins
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.relative_mins / 60,
            self.relative_mins % 60
        )
    }
}
