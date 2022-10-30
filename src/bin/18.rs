fn main() {
    let input = advent_of_code_2020::input::input("18");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut sum = 0;
    for line in input {
        sum += evaluate_part1(line);
    }
    println!("{}", sum);
}

fn part_two(input: &[String]) {
    let mut sum = 0;
    for line in input {
        let new_line = amend_line(&line.to_string());
        sum += evaluate_part1(&new_line);
    }
    println!("{}", sum);
}

fn amend_line(expression: &String) -> String {
    let mut index = 2;
    let mut final_exp = expression.clone();
    let mut len = final_exp.chars().count();

    while index < len {
        if final_exp.chars().nth(index).unwrap() == '+' {
            if final_exp.chars().nth(index - 2).unwrap() == ')' {
                let start_bracket = find_start_bracket(&final_exp, index - 2);
                final_exp.insert(start_bracket, '(');
            } else {
                final_exp.insert(index - 2, '(');
            }

            index += 1;

            if final_exp.chars().nth(index + 2).unwrap() == '(' {
                let end_bracket = find_end_bracket(&final_exp, index + 2);
                final_exp.insert(end_bracket + 1, ')');
            } else {
                final_exp.insert(index + 3, ')');
            }
        }

        index += 1;
        len = final_exp.chars().count();
    }

    final_exp
}

fn evaluate_part1(expression: &str) -> u64 {
    let mut curr_sum = 0;
    let len = expression.chars().count();
    let mut index = 2;

    if expression.chars().next().unwrap() == '(' {
        let end_bracket = find_end_bracket(&expression, 0);
        curr_sum += evaluate_part1(&expression[1..end_bracket]);
        index = end_bracket + 1;
    } else {
        curr_sum += expression.chars().nth(0).unwrap().to_digit(10).unwrap() as u64;
    }

    while index < len {
        if expression.chars().nth(index).unwrap() == ' ' {
            index += 1;
        } else if expression.chars().nth(index).unwrap() == '+' {
            index += 2;
            if expression.chars().nth(index).unwrap() == '(' {
                let end_bracket = find_end_bracket(&expression, index);
                curr_sum += evaluate_part1(&expression[index + 1..end_bracket]);
                index += end_bracket - index + 1;
            } else {
                curr_sum += expression.chars().nth(index).unwrap().to_digit(10).unwrap() as u64;
                index += 2;
            }
        } else if expression.chars().nth(index).unwrap() == '*' {
            index += 2;
            if expression.chars().nth(index).unwrap() == '(' {
                let end_bracket = find_end_bracket(&expression, index);
                curr_sum *= evaluate_part1(&expression[index + 1..end_bracket]);
                index += end_bracket - index + 1;
            } else {
                curr_sum *= expression.chars().nth(index).unwrap().to_digit(10).unwrap() as u64;
                index += 2;
            }
        }
    }

    curr_sum
}

fn find_end_bracket(expression: &str, i_first: usize) -> usize {
    let mut index = i_first;
    let mut counter = 1;

    while counter >= 1 {
        index += 1;
        match expression.chars().nth(index).unwrap() {
            '(' => counter += 1,
            ')' => counter -= 1,
            _ => (),
        }
    }

    index
}

fn find_start_bracket(expression: &str, i_last: usize) -> usize {
    let mut index = i_last;
    let mut counter = 1;

    while counter >= 1 {
        index -= 1;
        match expression.chars().nth(index).unwrap() {
            ')' => counter += 1,
            '(' => counter -= 1,
            _ => (),
        }
    }

    index
}
