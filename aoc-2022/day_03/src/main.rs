use std::collections::HashSet;
use std::iter::FromIterator;

extern crate common;
use common::file_io::read_lines;

fn part_01(fi_name: String) {
    let input = read_lines(fi_name);
    let mut total: i32 = 0;
    for line in input {
        let full_rucksack: Vec<char> = line.chars().collect();
        let num_items = full_rucksack.len()/2;

        let rucksack_1 = HashSet::from_iter(full_rucksack[..num_items].iter().cloned());
        let rucksack_2 = HashSet::from_iter(full_rucksack[num_items..].iter().cloned());
        let diff = intersect(vec![rucksack_1, rucksack_2]);
        for letter in diff {
            total += priority_score(&letter);
        }
    }
    println!("Part 1: {:?}", total);
}

fn part_02(fi_name: String) {
    let input = read_lines(fi_name);
    let mut total: i32 = 0;
    let mut elf_group: Vec<HashSet<char>> = vec![];
    for line in input {
        let full_rucksack: Vec<char> = line.chars().collect();
        elf_group.push(HashSet::from_iter(full_rucksack.iter().cloned()));
        if elf_group.len() == 3 {
            let diff = intersect(elf_group.clone());
            for letter in diff {
                total += priority_score(&letter);
            }
            elf_group.clear();
        }
    }
    println!("Part 2: {:?}", total);
}

fn intersect(mut rucksacks: Vec<HashSet<char>>) -> Vec<char> {
    let mut elf_badge = rucksacks.pop().unwrap();
    elf_badge.retain(|item| {
        rucksacks.iter().all(|rucksack| rucksack.contains(item))
    });
    elf_badge.into_iter().map(|x| x.clone()).collect()
}

fn priority_score(letter: &char) -> i32 {
    let alphabet = ('a'..='z').into_iter().collect::<Vec<char>>();
    let letter_lowercased = letter.to_lowercase().collect::<Vec<_>>()[0];
    let idx = alphabet.iter()
        .position(|&i| i == letter_lowercased).unwrap() as i32;
    if letter.is_uppercase() {
        return idx + 27;
    } else {
        return idx + 1;
    }
}

fn main() {
    part_01("puzzle.txt".to_string());
    part_02("puzzle.txt".to_string());
}
