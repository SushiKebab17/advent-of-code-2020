fn main() {
    let input = advent_of_code_2020::input::input("02");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut count = 0;
    'outer: for str in input {
        let mut split_main = str.split(":");
        let mut conditions = split_main.next().unwrap().split(" ");
        let password = split_main.next().unwrap();

        let mut num_conditions = conditions.next().unwrap().split("-");
        let lower: i32 = num_conditions.next().unwrap().parse().unwrap();
        let upper: i32 = num_conditions.next().unwrap().parse().unwrap();
        let condition_char: char = conditions.next().unwrap().parse().unwrap();

        let mut char_count = 0;
        for x in password.chars() {
            if condition_char == x {
                char_count += 1;
            }
            if char_count > upper {
                continue 'outer;
            }
        }
        if char_count >= lower {
            count += 1;
        }
    }
    println!("{}", count);
}

fn part_two(input: &[String]) {
    let mut count = 0;
    for str in input {
        let mut split_main = str.split(":");
        let mut conditions = split_main.next().unwrap().split(" ");
        let password = split_main.next().unwrap();

        let mut num_conditions = conditions.next().unwrap().split("-");
        let i1 = num_conditions.next().unwrap().parse::<usize>().unwrap();
        let i2 = num_conditions.next().unwrap().parse::<usize>().unwrap();
        let condition_char: char = conditions.next().unwrap().parse().unwrap();

        if (password.chars().nth(i1).unwrap() == condition_char)
            ^ (password.chars().nth(i2).unwrap() == condition_char)
        {
            count += 1;
        }
    }
    println!("{}", count);
}
