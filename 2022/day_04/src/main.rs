use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;

fn read_input(fi_name: String) -> Vec<String> {
    let file = File::open(fi_name).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse(input: Vec<String>, check_all: bool) -> usize {
    let mut overlaps = vec![];
    for line in input {
        let pairs: Vec<_> = line.split(',').collect();
        let group_1: Vec<_> = pairs_to_groups(pairs[0]);
        let group_2: Vec<_> = pairs_to_groups(pairs[1]);
        overlaps.push(find_overlaps((group_1, group_2), check_all));
    }
    overlaps.retain(|c| c.is_some());
    overlaps.len()
}

fn pairs_to_groups(pair: &str) -> Vec<i32> {
    pair.split('-')
        .map(|num| num.parse::<i32>().unwrap())
        .collect()
}

fn find_overlaps(groups: (Vec<i32>,Vec<i32>), check_all: bool) -> Option<Vec<i32>> {
    let (start_a, end_a) = (groups.0[0], groups.0[1]);
    let (start_b, end_b) = (groups.1[0], groups.1[1]);
    let set_a: HashSet<i32> = HashSet::from_iter(groups.0[0]..=groups.0[1]);
    let set_b: HashSet<i32> = HashSet::from_iter(groups.1[0]..=groups.1[1]);

    match check_all {
        true => {
            if set_a.contains(&start_b) && set_a.contains(&end_b) {
                return Some(set_a.intersection(&set_b)
                    .into_iter()
                    .map(|x| x.clone())
                    .collect());
            }
            if set_b.contains(&start_a) && set_b.contains(&end_a) {
                return Some(set_b.intersection(&set_a)
                    .into_iter()
                    .map(|x| x.clone())
                    .collect());
            }
        }
        false => {
            if set_a.contains(&start_b) || set_a.contains(&end_b) {
            return Some(set_a.intersection(&set_b)
                .into_iter()
                .map(|x| x.clone())
                .collect());
            }
            if set_b.contains(&start_a) || set_b.contains(&end_a) {
                return Some(set_b.intersection(&set_a)
                    .into_iter()
                    .map(|x| x.clone())
                    .collect());
            }
        }
    }
    return None
}

fn part_01(fi_name: String) {
    let input = read_input(fi_name);
    let num_overlaps = parse(input, true);
    println!("Part 1: {:?}", num_overlaps);
}

fn part_02(fi_name: String) {
    let input = read_input(fi_name);
    let num_overlaps = parse(input, false);
    println!("Part 2: {:?}", num_overlaps);
}

fn main() {
    part_01("puzzle.txt".to_string());
    part_02("puzzle.txt".to_string());
}
