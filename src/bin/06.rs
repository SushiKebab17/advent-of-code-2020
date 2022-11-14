fn main() {
    let input = advent_of_code_2020::input::input("06");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut answered_yes: Vec<char> = Vec::new();
    let mut sum = 0;

    for line in input {
        if !line.is_empty() {
            for c in line.chars() {
                if !answered_yes.contains(&c) {
                    answered_yes.push(c);
                }
            }
        } else {
            sum += answered_yes.len();
            answered_yes.clear();
        }
    }
    sum += answered_yes.len();
    println!("{}", sum);
}

fn part_two(input: &[String]) {
    let mut answered_yes: Vec<char> = Vec::new();
    let mut group_size = 0;
    let mut sum = 0;
    let mut line_num = 0;

    for line in input {
        line_num += 1;
        if !line.is_empty() {
            group_size += 1;
            for c in line.chars() {
                answered_yes.push(c);
            }
        } else {
            answered_yes.sort();
            let mut prev_char = '0';
            let mut count = 0;
            for c in &answered_yes {
                if prev_char == '0' {
                    count += 1;
                } else {
                    if c == &prev_char {
                        count += 1;
                    } else {
                        count = 1;
                    }
                }
                if count == group_size {
                    sum += 1;
                }
                prev_char = *c;
            }
            group_size = 0;
            answered_yes.clear();
        }
    }
    answered_yes.sort();
    let mut prev_char = '0';
    let mut count = 0;
    for c in &answered_yes {
        if prev_char == '0' {
            count += 1;
        } else {
            if c == &prev_char {
                count += 1;
            } else {
                count = 1;
            }
        }
        if count == group_size {
            sum += 1;
        }
        prev_char = *c;
    }
    println!("{}", sum);
}
