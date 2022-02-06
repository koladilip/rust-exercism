use std::cmp::Ordering;
use std::collections::HashMap;
enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl From<&str> for MatchResult {
    fn from(origin: &str) -> MatchResult {
        match origin {
            "win" => MatchResult::Win,
            "loss" => MatchResult::Loss,
            "draw" => MatchResult::Draw,
            _ => panic!("Invalida match result"),
        }
    }
}

impl MatchResult {
    fn reverse(&self) -> Self {
        match self {
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Loss => MatchResult::Win,
            MatchResult::Draw => MatchResult::Draw,
        }
    }
}

#[derive(Debug)]
struct Team {
    name: String,
    win: u8,
    loss: u8,
    draw: u8,
}

impl Team {
    fn new(name: String) -> Self {
        Team {
            name,
            win: 0,
            loss: 0,
            draw: 0,
        }
    }

    fn add_match(&mut self, result: &MatchResult) {
        use MatchResult::*;
        match result {
            Win => self.add_win(),
            Loss => self.add_loss(),
            Draw => self.add_draw(),
        }
    }

    fn add_win(&mut self) {
        self.win += 1
    }

    fn add_loss(&mut self) {
        self.loss += 1
    }

    fn add_draw(&mut self) {
        self.draw += 1
    }

    fn points(&self) -> u8 {
        self.win * 3 + self.draw
    }

    fn total_matches(&self) -> u8 {
        self.win + self.loss + self.draw
    }
}

fn add_match_result(score_table: &mut HashMap<String, Team>, match_result: &str) {
    let match_parts: Vec<&str> = match_result.split(';').collect();
    if match_parts.len() != 3 {
        return;
    }
    let home = match_parts[0].to_string();
    let away = match_parts[1].to_string();
    let result = match_parts[2].into();
    score_table
        .entry(home.clone())
        .or_insert(Team::new(home))
        .add_match(&result);
    score_table
        .entry(away.clone())
        .or_insert(Team::new(away))
        .add_match(&result.reverse());
}

fn make_table_row(team: &Team) -> String {
    format!(
        "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
        team.name,
        team.total_matches(),
        team.win,
        team.draw,
        team.loss,
        team.points(),
    )
}

pub fn tally(match_results: &str) -> String {
    let mut score_table: HashMap<String, Team> = HashMap::new();
    match_results.split('\n').for_each(|result| {
        add_match_result(&mut score_table, result);
    });
    let mut teams: Vec<Team> = score_table.into_values().collect();
    teams.sort_by(|a, b| {
        b.points()
            .cmp(&a.points())
            .then_with(|| a.name.cmp(&b.name))
    });
    vec![format!(
        "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
        "Team", "MP", "W", "D", "L", "P",
    )]
    .into_iter()
    .chain(teams.iter().map(make_table_row))
    .collect::<Vec<String>>()
    .join("\n")
}
