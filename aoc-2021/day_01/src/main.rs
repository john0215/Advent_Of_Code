extern crate common;
use common::file_io::read_lines;


fn count_increases(input: Vec<String>) -> i32 {
    let mut count_increase = 0;
    for idx in 1..input.len() {
        let current: i32 = input[idx].parse().unwrap();
        let previous: i32 = input[idx - 1].parse().unwrap();
        if current > previous {
            count_increase += 1;
        }
    }
    return count_increase;
}

fn count_sum_increases(input: Vec<String>) -> i32 {
    let mut count_increase = 0;
    let mut previous_sum = 0;
    for idx in 1..input.len()-1 {
        let current: i32 = input[idx].parse().unwrap();
        let previous: i32 = input[idx - 1].parse().unwrap();
        let future: i32 = input[idx + 1].parse().unwrap();
        let current_sum = current + previous + future;
        if idx > 1 && current_sum > previous_sum {
            count_increase += 1;
        }
        previous_sum = current_sum;
    }
    return count_increase;
}

fn part_01(fi_name: String) {
    let input = read_lines(fi_name);
    println!("{:?}", count_increases(input));
}

fn part_02(fi_name: String) {
    let input = read_lines(fi_name);
    println!("{:?}", count_sum_increases(input));
}


fn main() {
    part_01("puzzle.txt".to_string());
    part_02("puzzle.txt".to_string());
}
