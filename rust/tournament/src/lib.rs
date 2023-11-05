use crate::PlayResult::{DRAW, LOSS, WIN};
use std::cmp::Reverse;
use std::collections::HashMap;

type TeamName = String;

#[derive(Clone, Debug, Eq, PartialEq)]
struct TeamBook {
    team: TeamName,
    stats: TeamStat,
}

impl TeamBook {
    fn new(name: &str) -> Self {
        TeamBook {
            team: name.to_string(),
            stats: TeamStat::new(),
        }
    }

    fn record(&mut self, result: PlayResult) {
        match result {
            PlayResult::WIN => self.stats.wins += 1,
            PlayResult::LOSS => self.stats.losses += 1,
            PlayResult::DRAW => self.stats.draws += 1,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct TeamStat {
    wins: i32,
    losses: i32,
    draws: i32,
}

impl TeamStat {
    fn new() -> Self {
        TeamStat {
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }
}

impl TeamStat {
    fn points(&self) -> i32 {
        self.wins * 3 + self.draws
    }

    fn matches_played(&self) -> i32 {
        self.wins + self.losses + self.draws
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PlayResult {
    WIN,
    LOSS,
    DRAW,
}

impl PlayResult {
    fn opposite(&self) -> Self {
        match self {
            WIN => LOSS,
            LOSS => WIN,
            DRAW => DRAW,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlayBook {
    book: HashMap<TeamName, TeamBook>,
}

impl PlayBook {
    fn new() -> Self {
        PlayBook {
            book: HashMap::new(),
        }
    }

    fn update_from_line_record(&mut self, line: &str) {
        let (team1, team2, result) = parse_line_record(line);
        self.update(&team1, &team2, result);
    }

    fn update(&mut self, team1: &str, team2: &str, result: PlayResult) {
        self.book
            .entry(team1.to_string())
            .or_insert(TeamBook::new(team1))
            .record(result);

        self.book
            .entry(team2.to_string())
            .or_insert(TeamBook::new(team2))
            .record(result.opposite());
    }

    fn records(&self) -> Vec<TeamBook> {
        let mut records: Vec<TeamBook> = self.book.values().map(|c| c.clone()).collect();
        records.sort_by_key(|r| (Reverse(r.stats.points()), r.team.to_string()));
        records
    }

    fn print_stats(&self) -> String {
        let records = self.records();
        let mut result = String::new();
        result.push_str("Team                           | MP |  W |  D |  L |  P");
        for r in records.iter() {
            result.push_str("\n");
            result.push_str(&format!(
                "{: <31}| {: >2} | {: >2} | {: >2} | {: >2} | {: >2}",
                r.team,
                r.stats.matches_played(),
                r.stats.wins,
                r.stats.draws,
                r.stats.losses,
                r.stats.points()
            ));
        }

        result
    }
}

fn parse_line_record(line: &str) -> (String, String, PlayResult) {
    let comps: Vec<_> = line.split(';').collect();
    let result = match comps[2] {
        "win" => WIN,
        "loss" => LOSS,
        "draw" => DRAW,
        _ => panic!("Unexpected play result"),
    };

    (comps[0].to_string(), comps[1].to_string(), result)
}

pub fn tally(match_results: &str) -> String {
    let mut playbook = PlayBook::new();
    for line in match_results.lines() {
        playbook.update_from_line_record(&line);
    }
    playbook.print_stats()
}

#[cfg(test)]
mod test {
    use crate::PlayBook;
    use crate::PlayResult::LOSS;

    #[test]
    fn test_updates() {
        let mut playbook = PlayBook::new();

        println!("{:?}", playbook);
        playbook.update("sharks", "orcas", LOSS);

        println!("{:?}", playbook);

        let r = playbook.records();
        println!("{:?}", r);

        println!("{}", playbook.print_stats());
    }
}
