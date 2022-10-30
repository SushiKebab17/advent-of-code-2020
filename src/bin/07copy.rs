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
                .push(parent.clone());
        }
    }
    let mut accessed = HashSet::new();
    traverse1(&"shiny gold".to_string(), &rules, &mut accessed);
    println!("{}", accessed.len() - 1);
}

fn part_two(input: &[String]) {
    let mut rules = HashMap::new();
    for line in input {
        let (parent, child) = parse_line(line);
        rules.insert(parent, child);
    }
    let mut accessed = HashMap::new();
    println!(
        "{}",
        traverse2(&"shiny gold".to_string(), &rules, &mut accessed)
    );
}

fn traverse1(bag: &String, map: &HashMap<String, Vec<String>>, accessed: &mut HashSet<String>) {
    if accessed.insert(bag.to_string()) {
        if map.contains_key(bag) {
            for b in &map[bag] {
                traverse1(&b, &map, accessed);
            }
        }
    }
}

fn traverse2(
    bag: &String,
    map: &HashMap<String, Vec<Contents>>,
    cache: &mut HashMap<String, u32>,
) -> u32 {
    if !cache.contains_key(bag) {
        if let Some(bags) = map.get(bag) {
            let mut count = 0;
            for item in bags {
                count += item.num * (1 + traverse2(&item.colour, &map, cache));
            }
            return count;
        } else {
            return 0;
        }
    } else {
        return cache[bag];
    }
}

#[derive(Debug)]
struct Contents {
    num: u32,
    colour: String,
}

fn parse_line(line: &String) -> (String, Vec<Contents>) {
    let mut split_main = line.split(" ");
    let bag = split_main.next().unwrap().to_string() + " " + split_main.next().unwrap();
    split_main.nth(1);
    let mut contents = Vec::new();
    loop {
        let num = split_main.next().unwrap();

        if let Ok(num) = num.parse() {
            let contained_bag =
                split_main.next().unwrap().to_string() + " " + split_main.next().unwrap();
            contents.push(Contents {
                num: num,
                colour: contained_bag,
            });
        } else {
            break;
        }

        if split_main.next().unwrap().chars().last().unwrap() == '.' {
            break;
        }
    }
    (bag, contents)
}
