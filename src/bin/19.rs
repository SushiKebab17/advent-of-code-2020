use core::slice::Iter;
use regex::Regex;
use std::{collections::HashMap, time::Instant};

fn main() {
    let input = advent_of_code_2020::input::input("19");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let now = Instant::now();
    let (rules, mut lines) = parse(input);
    let regex = make_regex(&rules, 0) + r"\b";
    let re = Regex::new(&regex).unwrap();
    let mut total = 0;
    while let Some(line) = lines.next() {
        if re.is_match(line) {
            total += 1;
        }
    }
    println!("{}ms", now.elapsed().as_micros() as f64 / 1000.);
    println!("{}", total);
}

fn part_two(_input: &[String]) {}

fn parse(input: &[String]) -> (HashMap<u32, Vec<&str>>, Iter<String>) {
    let mut lines = input.iter();
    let mut rules = HashMap::new();
    let mut line = lines.next().unwrap();
    while !line.is_empty() {
        let mut split = line.split(": ");
        rules.insert(
            split.next().unwrap().parse::<u32>().unwrap(),
            split.next().unwrap().split(" | ").collect(),
        );
        line = lines.next().unwrap();
    }
    (rules, lines)
}

fn make_regex(map: &HashMap<u32, Vec<&str>>, rule: u32) -> String {
    let mut regex = String::new();
    let list = &map[&rule];
    if list[0].starts_with("\"") {
        return list[0]
            .strip_prefix("\"")
            .unwrap()
            .strip_suffix("\"")
            .unwrap()
            .to_string();
    } else if list.len() == 1 {
        for sub_rule in list[0].split(" ") {
            regex += "(";
            regex += &make_regex(map, sub_rule.parse().unwrap());
            regex += ")";
        }
    } else {
        for sub_rule in list[0].split(" ") {
            regex += "(";
            regex += &make_regex(map, sub_rule.parse().unwrap());
            regex += ")";
        }
        regex += "|";
        for sub_rule in list[1].split(" ") {
            regex += "(";
            regex += &make_regex(map, sub_rule.parse().unwrap());
            regex += ")";
        }
    }
    regex
}
