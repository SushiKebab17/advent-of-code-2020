use std::time::Instant;

fn main() {
    let input = advent_of_code_2020::input::input("09");
    part_one(&input);
    let now = Instant::now();
    part_two(&input);
    println!("{}ms", now.elapsed().as_millis());
}

fn part_one(input: &[String]) {
    let numbers: Vec<u64> = input.iter().map(|x| x.parse().unwrap()).collect();
    println!("{}", part_one_solve(&numbers));
}

fn part_two(input: &[String]) {
    let numbers: Vec<u64> = input.iter().map(|x| x.parse().unwrap()).collect();
    let num = part_one_solve(&numbers);

    for start_index in 0..numbers.len() {
        let mut end_index = start_index + 1;
        let mut sum = numbers[start_index] + numbers[end_index];
        while sum < num && end_index < numbers.len() {
            end_index += 1;
            sum += numbers[end_index];
        }
        if sum == num {
            let (mut min, mut max) = (numbers[start_index], numbers[start_index]);
            for num in &numbers[start_index..=end_index] {
                if num < &min {
                    min = *num;
                }
                if num > &max {
                    max = *num;
                }
            }
            println!("{}", min + max);
            return;
        }
    }
}

fn part_one_solve(numbers: &[u64]) -> u64 {
    for i in 25..numbers.len() {
        let mut check = false;
        for x in i - 25..i {
            for y in x + 1..i {
                if numbers[x] + numbers[y] == numbers[i] {
                    check = true;
                }
            }
        }
        if !check {
            return numbers[i];
        }
    }
    0
}
