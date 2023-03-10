extern crate common;
use common::file_io::read_lines;

#[derive(Default)]
struct CalorieCounts {
    elf_counts: Vec<i32>
}

impl CalorieCounts {
    fn parse(&mut self, input: Vec<String>) {
        let mut total_calories = 0;
    
        for line in input {
            if line == "" {
                self.elf_counts.push(total_calories);
                total_calories = 0;
            } else {
                let calories: i32 = line.parse().unwrap();
                total_calories += calories;
            }
        }
    }
    
    fn part_1(&mut self, fi_name: String) {
        let input = read_lines(fi_name);
        self.parse(input);
        self.elf_counts.sort();
        println!("Part 1:: Max Calories: {:?}", self.elf_counts.last().unwrap());
    }

    fn part_2(&mut self, fi_name: String) {
        let input = read_lines(fi_name);
        self.parse(input);
        self.elf_counts.sort();
        let top_3_summed: i32 = self.elf_counts.iter().rev().take(3).sum();
        println!("Part 2:: Top 3 Totals Summed: {:?}", top_3_summed);
    }
    
}

fn main() {
    let mut calorie_counts = CalorieCounts::default();
    calorie_counts.part_1("puzzle.txt".to_string());

    let mut calorie_counts = CalorieCounts::default();
    calorie_counts.part_2("puzzle.txt".to_string());
}
