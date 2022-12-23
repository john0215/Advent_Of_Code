use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::VecDeque;

#[derive(Default)]
struct Instructions {
    amount: u16,
    source: usize,
    destination: usize,
}
impl Instructions {
    fn parse(&mut self, line: String) {
        let line_split: Vec<&str> = line.split(' ').into_iter().collect();
        self.amount = line_split[1].parse().unwrap();
        self.source = line_split[3].parse().unwrap();
        self.destination = line_split[5].parse().unwrap();
    }
}

#[derive(Default, Clone)]
struct SupplyStacks {
    cargo_ships: Vec<VecDeque<char>>,
} 
impl SupplyStacks {
    fn parse(&mut self, input: Vec<String>, one_each: bool) {
        for (_idx, line) in input.iter().enumerate() {
            if !line.contains("move"){
                self.build_cargo_ships(line.clone());
            } else {
                let mut instructions = Instructions::default();
                instructions.parse(line.to_string());
                self.apply_instructions(instructions, one_each);
            }
        }
    }

    fn apply_instructions(&mut self, instructions: Instructions, one_each: bool) {
        let mut intermediate: VecDeque<char> = VecDeque::default();
        for _x in 0..instructions.amount{
            if one_each {
                let cargo = self.cargo_ships[instructions.source -1].pop_front();
                self.cargo_ships[instructions.destination -1].push_front(cargo.unwrap());
            } else {
                let cargo = self.cargo_ships[instructions.source -1].pop_front();
                intermediate.push_front(cargo.unwrap());
            }
        }
        if !one_each {
            for cargo in intermediate {
                self.cargo_ships[instructions.destination -1].push_front(cargo);
            }
        }
    }

    fn build_cargo_ships(&mut self, line: String){
        let mut count = 0;
        for (idx, letter) in line.chars().enumerate() {
            if idx == 1 || (idx+3) % 4 == 0 {
                if letter != ' ' && letter.is_alphabetic() {
                    self.cargo_ships[count].push_back(letter);
                }
                count += 1;
            }
        }
    }
}

fn run(input: Vec<String>, num_ships: usize, one_each: bool) -> String {
    let mut ss = SupplyStacks::default();
    ss.cargo_ships = vec![VecDeque::new(); num_ships];
    ss.parse(input, one_each);
    let mut top_crates = vec![];
    for ship in ss.cargo_ships {
        for cargo in ship{
            top_crates.push(cargo);
            break;
        }
    }
    top_crates.iter().collect::<String>()
}

fn part_01(fi_name: String, num_ships: usize) {
    let input = read_input(fi_name);
    let result = run(input, num_ships, true);
    println!("Part 1: {:?}",result);
}

fn part_02(fi_name: String, num_ships: usize) {
    let input = read_input(fi_name);
    let result = run(input, num_ships, false);
    println!("Part 2: {:?}",result);
}

fn read_input(fi_name: String) -> Vec<String> {
    let file = File::open(fi_name).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn main() {
    part_01("puzzle.txt".to_string(), 9);
    part_02("puzzle.txt".to_string(), 9);
}
