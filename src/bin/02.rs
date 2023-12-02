use std::io::{self, BufRead};

use sscanf::scanf;

struct GameSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameSet {
    fn empty() -> GameSet {
        GameSet { red: 0, green: 0, blue: 0 }
    }

    fn parse(line: &str) -> GameSet {
        let parts = line.split(",").collect::<Vec<_>>();

        parts.iter()
            .map(|part| part.trim())
            .fold(
                GameSet { blue: 0, red: 0, green: 0 },
                |mut game_set, part| {
                    if let Ok(red) = scanf!(part, "{usize} red") {
                        game_set.red = red;
                    } else if let Ok(green) = scanf!(part, "{usize} green") {
                        game_set.green = green;
                    } else if let Ok(blue) = scanf!(part, "{usize} blue") {
                        game_set.blue = blue;
                    }

                    game_set
                }
            )
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }

    fn max(&self, other: &GameSet) -> GameSet {
        GameSet {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn is_feasible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}

struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

impl Game {
    fn parse(line: &str) -> Option<Game> {
        let (id, rest) = scanf!(line, "Game {usize}: {String}").ok()?;
        let sets = rest.split(";")
            .map(|set| GameSet::parse(set.trim()))
            .collect();

        Some(Game { id, sets })
    }

    fn iter(&self) -> impl Iterator<Item=&GameSet> + '_ {
        self.sets.iter()
    }

    fn is_feasible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.sets.iter().all(|set| set.is_feasible(red, green, blue))
    }
}

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().filter_map(Result::ok).collect::<Vec<_>>();
    let games = lines.iter().filter_map(|line| Game::parse(&line)).collect::<Vec<_>>();

    println!("{}", games.iter().filter(|game| game.is_feasible(12, 13, 14)).map(|game| game.id).sum::<usize>());
    println!("{}", games.iter().map(|game| game.iter().fold(GameSet::empty(), |acc, game_set| acc.max(game_set)).power()).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&'static str; 5] = [
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    #[test]
    fn _01() {
        assert_eq!(
            LINES.iter().filter_map(|line| Game::parse(line)).filter(|game| game.is_feasible(12, 13, 14)).map(|game| game.id).sum::<usize>(),
            8
        );
    }

    #[test]
    fn _02() {
        let games = LINES.iter().filter_map(|line| Game::parse(line)).collect::<Vec<_>>();

        assert_eq!(
            games.iter().map(|game| game.iter().fold(GameSet::empty(), |acc, game_set| acc.max(game_set)).power()).sum::<usize>(),
            2286
        );
    }
}
