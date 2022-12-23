use std::{collections::{HashSet, VecDeque}};

extern crate common;
use common::file_io::read_lines;

#[derive(Default, Clone, Copy)]
struct TuningTrouble {
    window_amt: usize,
    marker: i32,
}

impl TuningTrouble {
    fn check_signal(&mut self, signal: String) {
        let mut signal_chunk = VecDeque::new();
        for (_i, value) in signal.chars().enumerate() {
            signal_chunk.push_back(value);
            // println!("{:?}", signal_chunk);
            if signal_chunk.len() == self.window_amt {
                if self.is_unique(signal_chunk.clone()) {
                    self.marker += self.window_amt as i32;
                    break;
                }
                signal_chunk.pop_front();
                self.marker += 1
            }
        }
    }

    fn is_unique(&mut self, signal_chunk: VecDeque<char>) -> bool {
        let signal_set: HashSet<&char> = HashSet::from_iter(signal_chunk.iter());
        // println!("{:?}", signal_set);
        signal_set.len() == self.window_amt
    }
}

fn part_01(fi_name: String) {
    let signal = read_lines(fi_name);
    let mut tt = TuningTrouble {window_amt: 4, marker: 0};
    tt.check_signal(signal[0].clone());
    println!("Part 1: {:?}", tt.marker);
}

fn part_02(fi_name: String) {
    let signal = read_lines(fi_name);
    let mut tt = TuningTrouble {window_amt: 14, marker: 0};
    tt.check_signal(signal[0].clone());
    println!("Part 2: {:?}", tt.marker);
}

fn main() {
    part_01("puzzle.txt".to_string());
    part_02("puzzle.txt".to_string());
}
