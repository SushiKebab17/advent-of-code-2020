use std::{collections::HashMap, time::Instant};

fn main() {
    let input = advent_of_code_2020::input::input("15");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let now = Instant::now();
    let (table, prev_turn_num, turn) = parse_input(&input[0]);
    let ans = eval_nth_num(table, prev_turn_num, turn, 2020);
    println!("{}ms", now.elapsed().as_micros() as f64 / 1000.);
    println!("{}", ans);
}

fn part_two(input: &[String]) {
    let now = Instant::now();
    let (table, prev_turn_num, turn) = parse_input(&input[0]);
    let ans = eval_nth_num(table, prev_turn_num, turn, 30000000);
    println!("{}ms", now.elapsed().as_micros() as f64 / 1000.);
    println!("{}", ans);
}

fn parse_input(input: &String) -> (HashMap<u32, u32>, u32, u32) {
    let mut table: HashMap<u32, u32> = HashMap::new();
    let count = input.split(",").count() as u32;
    let mut split = input.split(",");
    for i in 1..count {
        table.insert(split.next().unwrap().parse().unwrap(), i as u32);
    }
    (table, split.next().unwrap().parse().unwrap(), count + 1)
}

fn eval_nth_num(
    mut table: HashMap<u32, u32>,
    mut prev_turn_num: u32,
    mut turn: u32,
    n: u32,
) -> u32 {
    while turn <= n {
        let curr_num = if !table.contains_key(&prev_turn_num) {
            0
        } else {
            turn - table.get(&prev_turn_num).unwrap() - 1
        };
        table.insert(prev_turn_num, turn - 1);
        prev_turn_num = curr_num;
        turn += 1;
    }
    prev_turn_num
}
