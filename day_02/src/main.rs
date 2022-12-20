use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Default)]
struct Submarine {
    distance: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    fn change_distance(&mut self, aim_enabled: bool,  amount: i32) {
        self.distance += amount;
        if aim_enabled == true {
            self.depth += self.aim * amount;
        }
    }

    fn change_depth(&mut self, aim_enabled: bool,  amount: i32) {
        if aim_enabled == true {
            self.aim += amount;
        } else {
            self.depth += amount;
        }
    }

    fn parse_commands(&mut self, input: Vec<String>, aim_enabled: bool) {
        for command in input {
            let command_split: Vec<_> = command.split(' ').collect();
            let amount: i32 = command_split[1].parse().unwrap();
            match command_split[0] {
                "forward" => self.change_distance(aim_enabled, amount),
                "up" => self.change_depth(aim_enabled, amount * -1),
                "down" => self.change_depth(aim_enabled, amount),
                _ => panic!("Unhandled command: {:?}",command)
            }
        }
    }

    fn part_1(&mut self, fi_name: String) {
        let input = read_input(fi_name);
        self.parse_commands(input, false);
        println!("Part 1: {:?}", self.distance * self.depth);
    }

    fn part_2(&mut self, fi_name: String) {
        let input = read_input(fi_name);
        self.parse_commands(input, true);
        println!("Part 2: {:?}", self.distance * self.depth);
    }
    
}

fn read_input(fi_name: String) -> Vec<String> {
    let file = File::open(fi_name).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let mut submarine = Submarine::default();
    submarine.part_1("puzzle.txt".to_string());
    
    let mut submarine = Submarine::default();
    submarine.part_2("puzzle.txt".to_string());
}
