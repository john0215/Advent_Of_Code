use core::panic;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn parse(letter: &str) -> Self {
        match letter {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Unrecognized letter: {:?}",letter)
        }
    }

    fn point(choice: RPS) -> i32 {
        match choice {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}
#[derive(Default)]
struct Game {
    score: i32,
    total_score: i32,
}

impl Game {
    fn parse(&mut self, input: Vec<String>, use_rules: bool) {
        for line in input {
            let strategy_split: Vec<_> = line.split(' ').collect();
            let opponent = RPS::parse(strategy_split[0]);
            if use_rules == true {
                let player = RPS::parse(strategy_split[1]);
                self.score = self.determined_by_rules(opponent, player) + RPS::point(player);
            } else {
                self.score = self.determined_by_outcome(opponent, strategy_split[1]);
            }
            self.total_score += self.score;
        }
    }

    fn determined_by_rules(&mut self, opponent: RPS, player:RPS) -> i32 {
        let outcome = (opponent, player);
        match outcome {
            (RPS::Scissors, RPS::Rock) | (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) => WIN,
            (RPS::Scissors, RPS::Scissors) | (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) => DRAW,
            _ => LOSS
        }
    }

    fn determined_by_outcome(&mut self, opponent:RPS, outcome: &str) -> i32 {
        let outcome = (opponent, outcome);
        match outcome {
            (RPS::Rock,"X") => LOSS + RPS::point(RPS::Scissors),
            (RPS::Paper, "X") => LOSS + RPS::point(RPS::Rock),
            (RPS::Scissors, "X") => LOSS + RPS::point(RPS::Paper),
            (RPS::Rock,"Z") => WIN + RPS::point(RPS::Paper),
            (RPS::Paper,"Z") => WIN + RPS::point(RPS::Scissors),
            (RPS::Scissors,"Z") => WIN + RPS::point(RPS::Rock),
            _ => DRAW + RPS::point(opponent)
        }
    }
}

fn read_input(fi_name: String) -> Vec<String> {
    let file = File::open(fi_name).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part_01(fi_name: String) {
    let input = read_input(fi_name);
    let mut game = Game::default();
    game.parse(input, true);
    println!("Part 1: {:?}",game.total_score);
}

fn part_02(fi_name: String) {
    let input = read_input(fi_name);
    let mut game = Game::default();
    game.parse(input, false);
    println!("Part 2: {:?}",game.total_score);
}

fn main() {
    part_01("puzzle.txt".to_string());
    part_02("puzzle.txt".to_string());
}
