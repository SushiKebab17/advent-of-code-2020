use std::{time::Instant, u64};

fn main() {
    let input = advent_of_code_2020::input::input("13");
    let now = Instant::now();
    part_one(&input);
    println!("{}ms", now.elapsed().as_micros() as f64 / 1000.);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut lines = input.iter();
    let time: u64 = lines.next().unwrap().parse().unwrap();
    let buses: Vec<u64> = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|x| x != &"x")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut lowest = u64::MAX;
    let mut bus_num = 0;
    for bus in &buses {
        let wait = bus - (time % bus);
        if wait < lowest {
            lowest = wait;
            bus_num = *bus;
        }
    }
    println!("{}", lowest * bus_num);
}

fn part_two(input: &[String]) {
    let mut timestamp = 0;
    let mut step = 1;
    for (i, id) in input[1].split(",").enumerate() {
        if let Ok(num) = id.parse::<usize>() {
            loop {
                if (timestamp + i) % num == 0 {
                    break;
                }
                timestamp += step;
            }
            step *= num;
        }
    }
    println!("{}", timestamp);
}
