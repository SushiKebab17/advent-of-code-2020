use std::{collections::HashSet, fmt::Debug, time::Instant};

fn main() {
    let input = advent_of_code_2020::input::input("16");
    part_two(&input, part_one(&input));
}

fn part_one(input: &[String]) -> HashSet<usize> {
    let now = Instant::now();
    let mut lines = input.iter();
    let mut line = lines.next().unwrap();
    let mut all_fields = Vec::new();
    while line != "" {
        all_fields.push(Field::parse(line));
        line = lines.next().unwrap();
    }
    lines.nth(3);
    let mut total = 0;
    let mut lines_with_invalids = HashSet::new();
    for (i, line) in lines.enumerate() {
        'num: for num in line.split(",").map(|x| x.parse().unwrap()) {
            for ranges in &all_fields {
                if ranges.contains(num) {
                    continue 'num;
                }
            }
            lines_with_invalids.insert(i);
            total += num;
        }
    }
    println!("{}ms", now.elapsed().as_micros() as f64 / 1000.);
    println!("{}", total);
    lines_with_invalids
}

fn part_two(input: &[String], lines_with_invalids: HashSet<usize>) {
    let mut lines = input.iter();
    let mut line = lines.next().unwrap();
    let mut all_fields = Vec::new();
    while line != "" {
        all_fields.push(Field::parse(line));
        line = lines.next().unwrap();
    }
    line = lines.nth(1).unwrap();
    let mut possible_values = Vec::new();
    let my_ticket: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
    for &num in &my_ticket {
        let mut set = HashSet::new();
        for (i_field, field) in all_fields.iter().enumerate() {
            if field.contains(num) {
                set.insert(i_field);
            }
        }
        possible_values.push(set);
    }
    lines.nth(1);
    for (line_index, line) in lines.enumerate() {
        if lines_with_invalids.contains(&line_index) {
            continue;
        }
        for (i, value) in line.split(",").map(|x| x.parse().unwrap()).enumerate() {
            possible_values[i].retain(|&j| all_fields[j].contains(value));
        }
    }
    let mut locked_in_values = vec![usize::MAX; 6];
    while let Some((pos, value)) = hashset_of_size_1(&possible_values) {
        for set in possible_values.iter_mut() {
            set.remove(&value);
        }
        if value < 6 {
            locked_in_values[value] = pos;
        }
    }
    let mut total = 1;
    for pos in locked_in_values {
        total *= my_ticket[pos];
    }
    println!("{}", total);
}

fn hashset_of_size_1(possible_values: &[HashSet<usize>]) -> Option<(usize, usize)> {
    for (i, set) in possible_values.iter().enumerate() {
        if set.len() == 1 {
            return Some((i, *set.iter().next().unwrap()));
        }
    }
    None
}

struct Field<'a> {
    name: &'a str,
    range_1: (usize, usize),
    range_2: (usize, usize),
}

impl Field<'_> {
    fn parse(line: &str) -> Field {
        let split = line.split(":").nth(1).unwrap();
        let range_1 = split.split(" ").nth(1).unwrap();
        let range_2 = split.split(" ").nth(3).unwrap();
        Field {
            name: (line.split(":").nth(0).unwrap()),
            range_1: (
                range_1.split("-").nth(0).unwrap().parse().unwrap(),
                range_1.split("-").nth(1).unwrap().parse().unwrap(),
            ),
            range_2: (
                range_2.split("-").nth(0).unwrap().parse().unwrap(),
                range_2.split("-").nth(1).unwrap().parse().unwrap(),
            ),
        }
    }

    fn contains(&self, num: usize) -> bool {
        (num >= self.range_1.0 && num <= self.range_1.1)
            || (num >= self.range_2.0 && num <= self.range_2.1)
    }
}

impl Debug for Field<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: ({}-{}) or ({}-{})",
            self.name, self.range_1.0, self.range_1.1, self.range_2.0, self.range_2.1
        );
        Ok(())
    }
}
