use std::time::Instant;

fn main() {
    let input = advent_of_code_2020::input::input("10");
    part_one(&input);
    let now = Instant::now();
    part_two(&input);
    println!(
        "Original Suket's - {}ms",
        now.elapsed().as_micros() as f64 / 1000.
    );
}

fn part_one(input: &[String]) {
    let numbers = parse(input);
    let (mut ones, mut threes) = (0, 0);
    for i in 1..numbers.len() {
        match numbers[i] - numbers[i - 1] {
            1 => ones += 1,
            _ => threes += 1,
        }
    }
    println!("{}", ones * threes);
}

fn part_two(input: &[String]) {
    let numbers = parse(input);
    let mut total: u64 = 1;
    let mut start = 0;
    for i in 1..numbers.len() {
        if numbers[i] - numbers[i - 1] == 3 {
            match i - start {
                3 => total *= 2,
                4 => total *= 4,
                5 => total *= 7,
                _ => (),
            }
            start = i;
        }
    }
    println!("{}", total);
}

fn parse(input: &[String]) -> Vec<u32> {
    let mut numbers: Vec<u32> = input.iter().map(|x| x.parse().unwrap()).collect();
    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + 3);
    numbers
}
