fn main() {
    let input = advent_of_code_2020::input::input("04");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut count = 0;
    let passports = get_passports(input);
    for passport in passports {
        let mut passport_components = Vec::new();
        for component in passport.split(" ") {
            passport_components.push(component);
        }

        match passport_components.len() {
            8 => count += 1,
            7 => {
                let mut cid_exists = false;
                for component in passport_components {
                    if &component[..3] == "cid" {
                        cid_exists = true;
                        break;
                    }
                }
                if !cid_exists {
                    count += 1
                }
            }
            _ => (),
        }
    }
    println!("{}", count);
}

fn part_two(input: &[String]) {
    let mut count = 0;
    let passports = get_passports(input);
    for passport in passports {
        if is_valid(passport) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn get_passports(input: &[String]) -> Vec<String> {
    let mut current_passport = String::new();
    let mut passports = Vec::new();

    for line in input {
        if !line.is_empty() {
            current_passport += line;
            current_passport += " ";
        } else {
            current_passport.pop();
            passports.push(current_passport);
            current_passport = String::new();
        }
    }
    passports
}

fn is_valid(passport: String) -> bool {
    let mut passport_components = Vec::new();
    for component in passport.split(" ") {
        passport_components.push(component);
    }
    match passport_components.len() {
        8 => (),
        7 => {
            for component in &passport_components {
                if &component[..3] == "cid" {
                    return false;
                }
            }
        }
        _ => {
            return false;
        }
    }

    for comp in passport_components {
        let tag = &comp[..3];

        if tag == "byr" {
            if comp[4..].parse::<u64>().unwrap() < 1920 || comp[4..].parse::<u64>().unwrap() > 2002
            {
                return false;
            }
        } else if tag == "iyr" {
            if comp[4..].parse::<u64>().unwrap() < 2010 || comp[4..].parse::<u64>().unwrap() > 2020
            {
                return false;
            }
        } else if tag == "eyr" {
            if comp[4..].parse::<u64>().unwrap() < 2020 || comp[4..].parse::<u64>().unwrap() > 2030
            {
                return false;
            }
        } else if tag == "hgt" {
            let len = comp.len();
            if &comp[len - 2..len] == "cm" {
                if comp[4..len - 2].parse::<u64>().unwrap() < 150
                    || comp[4..len - 2].parse::<u64>().unwrap() > 193
                {
                    return false;
                }
            } else if &comp[len - 2..len] == "in" {
                if comp[4..len - 2].parse::<u64>().unwrap() < 59
                    || comp[4..len - 2].parse::<u64>().unwrap() > 76
                {
                    return false;
                }
            } else {
                return false;
            }
        } else if tag == "hcl" {
            if comp.chars().nth(4).unwrap() != '#' {
                return false;
            }

            if comp[5..].chars().count() != 6 {
                return false;
            }

            let hex_chars = [
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
            ];
            for c in comp[5..].chars() {
                if !hex_chars.contains(&c) {
                    return false;
                }
            }
        } else if tag == "ecl" {
            let colours = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            if !colours.contains(&&comp[4..]) {
                return false;
            }
        } else if tag == "pid" {
            if comp[4..].chars().count() != 9 {
                return false;
            }

            let dec_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
            for c in comp[4..].chars() {
                if !dec_chars.contains(&c) {
                    return false;
                }
            }
        }
    }
    true
}
