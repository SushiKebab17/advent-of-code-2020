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
    let (mut rules, mut lines) = parse(input);
    let regex = make_regex(&mut rules, 0) + r"\b";
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

fn parse(input: &[String]) -> (HashMap<u32, Rule>, Iter<String>) {
    let mut lines = input.iter();
    let mut rules = HashMap::new();
    let mut line = lines.next().unwrap();
    while !line.is_empty() {
        let mut split = line.split(": ");
        rules.insert(
            split.next().unwrap().parse().unwrap(),
            Rule::new(split.next().unwrap()),
        );
        line = lines.next().unwrap();
    }
    (rules, lines)
}

fn make_regex(map: &mut HashMap<u32, Rule>, rule: u32) -> String {
    let mut regex = String::new();
    match &map[&rule] {
        Rule::Regex(c) => return c.clone(),
        Rule::SubRule(list) => {
            let list = list.clone();
            for &sub_rule in &list[0] {
                regex += "(";
                regex += &make_regex(map, sub_rule);
                regex += ")";
            }
            if let Some(list_2) = list.get(1) {
                regex += "|";
                for &sub_rule in list_2 {
                    regex += "(";
                    regex += &make_regex(map, sub_rule);
                    regex += ")";
                }
            }
        }
    }
    *map.get_mut(&rule).unwrap() = Rule::Regex(regex.clone());
    regex
}

enum Rule {
    SubRule(Vec<Vec<u32>>),
    Regex(String),
}

impl Rule {
    fn new(input: &str) -> Rule {
        if input.starts_with("\"") {
            Rule::Regex(
                input
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\"")
                    .unwrap()
                    .to_string(),
            )
        } else {
            let mut patterns = Vec::new();
            for pattern in input.split(" | ") {
                let sub_patterns: Vec<u32> =
                    pattern.split(" ").map(|x| x.parse().unwrap()).collect();
                patterns.push(sub_patterns);
            }
            Rule::SubRule(patterns)
        }
    }
}
