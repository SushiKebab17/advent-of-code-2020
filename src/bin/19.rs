use core::slice::Iter;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = advent_of_code_2020::input::input("19");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let (mut rules, mut lines) = parse(input);
    println!("{}", evaluate_total(&mut rules, &mut lines, 0));
}

fn part_two(input: &[String]) {
    let (mut rules, mut lines) = parse(input);
    rules.insert(8, Rule::new("42 | 42 8"));
    rules.insert(11, Rule::new("42 31 | 42 11 31"));
    println!("{}", evaluate_total(&mut rules, &mut lines, 4));
}

fn evaluate_total(rules: &mut HashMap<u32, Rule>, lines: &mut Iter<String>, depth: u32) -> u32 {
    let regex = format!("^{}$", make_regex(rules, 0, depth));
    let re = Regex::new(&regex).unwrap();
    let mut total = 0;
    while let Some(line) = lines.next() {
        if re.is_match(line) {
            total += 1;
        }
    }
    total
}

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

fn make_regex(map: &mut HashMap<u32, Rule>, rule: u32, mut max_depth: u32) -> String {
    let mut regex = String::new();
    match &map[&rule].clone() {
        Rule::Regex(c) => return c.clone(),
        Rule::SubRule(list) => {
            for &sub_rule in &list[0] {
                regex += "(";
                regex += &make_regex(map, sub_rule, max_depth);
                regex += ")";
            }
            if let Some(list_2) = list.get(1) {
                if !list_2.contains(&rule) || max_depth >= 1 {
                    if max_depth >= 1 {
                        max_depth -= 1;
                    }
                    regex += "|";
                    for &sub_rule in list_2 {
                        regex += "(";
                        regex += &make_regex(map, sub_rule, max_depth);
                        regex += ")";
                    }
                }
            }
        }
    }
    *map.get_mut(&rule).unwrap() = Rule::Regex(regex.clone());
    regex
}

#[derive(Clone, Debug)]
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
