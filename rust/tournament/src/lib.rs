use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default)]
struct Results {
    played: u8,
    won: u8,
    drawn: u8,
    lost: u8,
    points: u8,
}

impl Results {
    fn add(self: &mut Self, result: &str) {
        self.played += 1;
        match result {
            "win" => self.won += 1,
            "loss" => self.lost += 1,
            "draw" => self.drawn += 1,
            _ => unreachable!(),
        }
        self.points = self.won * 3 + self.drawn
    }
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.played, self.won, self.drawn, self.lost, self.points
        )
    }
}

fn flip(result: &str) -> &'static str {
    match result {
        "win" => "loss",
        "loss" => "win",
        "draw" => "draw",
        _ => unreachable!(),
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<_, Results> = HashMap::new();
    for line in match_results.lines() {
        if let [team1, team2, result] = line.split(';').collect::<Vec<_>>()[..] {
            teams.entry(team1).or_default().add(result);
            teams.entry(team2).or_default().add(flip(result));
        }
    }

    let mut ordered: Vec<_> = teams.into_iter().collect();
    ordered.sort_by(|(n1, r1), (n2, r2)| r2.points.cmp(&r1.points).then(n1.cmp(&n2)));

    let mut strings: Vec<_> = ordered
        .iter()
        .map(|(name, results)| format!("{:30} {}", name, results))
        .collect();
    strings.insert(0, format!("{:30} | MP |  W |  D |  L |  P", "Team"));
    strings.join("\n")
}
