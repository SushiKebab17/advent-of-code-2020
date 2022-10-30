fn main() {
    let input = advent_of_code_2020::input::input("01");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut nums: Vec<i32> = Vec::new();
    for n in input {
        nums.push(n.parse().unwrap());
    }

    for &x in &nums {
        for &y in &nums {
            if x + y == 2020 {
                println!("{}", x * y);
            }
        }
    }
}

fn part_two(input: &[String]) {
    let mut nums: Vec<i32> = Vec::new();
    for n in input {
        nums.push(n.parse().unwrap());
    }

    for &x in &nums {
        for &y in &nums {
            for &z in &nums {
                if x + y + z == 2020 {
                    println!("{}", x * y * z);
                }
            }
        }
    }
}
