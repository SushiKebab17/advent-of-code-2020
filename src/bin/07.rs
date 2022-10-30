use std::collections::{HashMap, HashSet};

fn main() {
    let input = advent_of_code_2020::input::input("07");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut rules = HashMap::new();
    for line in input {
        let (parent, child) = parse_line(line);
        for bag_type in child {
            rules
                .entry(bag_type.colour)
                .or_insert(Vec::new())
                .push(parent);
        }
    }
    let mut accessed = HashSet::new();
    traverse1("shiny gold", &rules, &mut accessed);
    println!("{}", accessed.len() - 1);
}

fn part_two(input: &[String]) {
    let mut rules = HashMap::new();
    for line in input {
        let (parent, child) = parse_line(line);
        rules.insert(parent, child);
    }
    let mut accessed = HashMap::new();
    println!("{}", traverse2("shiny gold", &rules, &mut accessed));
}

fn traverse1<'a>(
    bag: &'a str,
    map: &HashMap<&'a str, Vec<&'a str>>,
    accessed: &mut HashSet<&'a str>,
) {
    if accessed.insert(bag) {
        if map.contains_key(bag) {
            for b in &map[bag] {
                traverse1(b, map, accessed);
            }
        }
    }
}

fn traverse2<'a>(
    bag: &'a str,
    map: &HashMap<&'a str, Vec<Contents<'a>>>,
    cache: &mut HashMap<&'a str, u32>,
) -> u32 {
    if !cache.contains_key(bag) {
        if let Some(bags) = map.get(bag) {
            let mut count = 0;
            for item in bags {
                count += item.num * (1 + traverse2(item.colour, map, cache));
            }
            cache.insert(bag, count);
            count
        } else {
            cache.insert(bag, 0);
            0
        }
    } else {
        cache[bag]
    }
}

#[derive(Debug)]
struct Contents<'a> {
    num: u32,
    colour: &'a str,
}

fn parse_line(line: &String) -> (&str, Vec<Contents>) {
    let mut split_main = line.split(", ");
    let mut sub_split = split_main.next().unwrap().split(" bags contain ");
    let bag = sub_split.next().unwrap();
    let first_bag = sub_split.next().unwrap();
    let mut contents = Vec::new();

    let mut index = first_bag.find(" bag").unwrap();
    let num = first_bag.split(" ").next().unwrap();
    if let Ok(num) = num.parse() {
        contents.push(Contents {
            num: (num),
            colour: (&first_bag[2..index]),
        });
    }

    for item in split_main {
        let num = item.chars().nth(0).unwrap().to_digit(10).unwrap();
        index = item.find(" bag").unwrap();
        let contained_bag = &item[2..index];
        contents.push(Contents {
            num: (num),
            colour: (contained_bag),
        });
    }

    (bag, contents)
}
