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

fn parse(input: &[String]) -> (HashMap<u32, Rule>, Iter<String>) {
    let mut lines = input.iter();
    let mut rules = HashMap::new();
    let mut line = lines.next().unwrap();
    while !line.is_empty() {
        let mut split = line.split(": ");
        let rule_num: u32 = split.next().unwrap().parse().unwrap();
        let sub_rules = split.next().unwrap();
        let rule = if sub_rules.starts_with("\"") {
            Rule::Regex(
                sub_rules
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\"")
                    .unwrap()
                    .to_string(),
            )
        } else {
            let mut patterns = Vec::new();
            for pattern in sub_rules.split(" | ") {
                let mut sub_patterns = Vec::new();
                for sub_pat in pattern.split(" ") {
                    sub_patterns.push(sub_pat.parse().unwrap());
                }
                patterns.push(sub_patterns);
            }
            Rule::SubRule(patterns)
        };
        rules.insert(rule_num, rule);
        line = lines.next().unwrap();
    }
    (rules, lines)
}

fn make_regex(map: &HashMap<u32, Rule>, rule: u32) -> String {
    let mut regex = String::new();
    let pattern = &map[&rule];
    match pattern {
        Rule::Regex(c) => return c.to_string(),
        Rule::SubRule(list) => {
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
    regex
}

enum Rule {
    SubRule(Vec<Vec<u32>>),
    Regex(String),
}
