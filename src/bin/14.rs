use std::{collections::HashMap, time::Instant};

fn main() {
    let input = advent_of_code_2020::input::input("14");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut mask = "";
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for line in input {
        if &line[..4] == "mask" {
            mask = &line[7..];
        } else {
            let mem_addr = &line.split("]").next().unwrap()[4..].parse().unwrap();
            let num = &line.split(" = ").skip(1).next().unwrap().parse().unwrap();
            memory.insert(*mem_addr, apply_mask_one(mask, *num));
        }
    }
    println!("{}", memory.values().sum::<u64>());
}

fn part_two(input: &[String]) {
    let mut now = Instant::now();
    let mut mask = "";
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for line in input {
        if &line[..4] == "mask" {
            mask = &line[7..];
        } else {
            let generic_address =
                apply_mask_two(mask, line.split("]").next().unwrap()[4..].parse().unwrap());
            let num = &line.split(" = ").skip(1).next().unwrap().parse().unwrap();
            let mut addresses: Vec<u64> = Vec::new();
            get_addresses(&generic_address, &mut addresses);
            for addr in addresses {
                memory.insert(addr, *num);
            }
        }
    }
    println!("{}ms", now.elapsed().as_micros() as f64 / 1000.);
    println!("{}", memory.values().sum::<u64>());
}

fn apply_mask_one(mask: &str, num: u64) -> u64 {
    let mask_or = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
    let mask_and = u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
    (num | mask_or) & mask_and
}

fn apply_mask_two(mask: &str, addr: u64) -> String {
    let mut addr_in_bin = format!("{:036b}", addr);
    for i in 0..36 {
        let ith_char = &mask[i..i + 1];
        if ith_char == "X" || ith_char == "1" {
            addr_in_bin.replace_range(i..i + 1, ith_char);
        }
    }
    return addr_in_bin;
}

fn get_addresses(generic: &str, addresses: &mut Vec<u64>) {
    if let Some(first) = generic.find("X") {
        get_addresses(
            &(generic[0..first].to_owned() + "0" + &generic[first + 1..36]),
            addresses,
        );
        get_addresses(
            &(generic[0..first].to_owned() + "1" + &generic[first + 1..36]),
            addresses,
        );
    } else {
        addresses.push(u64::from_str_radix(generic, 2).unwrap());
    }
}
