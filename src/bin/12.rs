fn main() {
    let input = advent_of_code_2020::input::input("12");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let (mut east, mut north, mut facing) = (0, 0, 1);
    for line in input {
        let action = line.chars().next().unwrap();
        let number: i32 = line[1..].parse().unwrap();
        match action {
            'N' => north += number,
            'S' => north -= number,
            'E' => east += number,
            'W' => east -= number,
            'L' => facing = (facing - (number / 90)).rem_euclid(4),
            'R' => facing = (facing + (number / 90)).rem_euclid(4),
            'F' => match facing {
                0 => north += number,
                1 => east += number,
                2 => north -= number,
                3 => east -= number,
                _ => (),
            },
            _ => (),
        }
    }
    println!("{}", east.abs() + north.abs());
}

fn part_two(input: &[String]) {
    let (mut w_east, mut w_north) = (10, 1);
    let (mut east, mut north) = (0, 0);
    for line in input {
        let action = line.chars().next().unwrap();
        let number: i32 = line[1..].parse().unwrap();
        match action {
            'N' => w_north += number,
            'S' => w_north -= number,
            'E' => w_east += number,
            'W' => w_east -= number,
            'L' | 'R' => match (action, number) {
                ('L', 90) | ('R', 270) => {
                    let temp = w_east;
                    w_east = -w_north;
                    w_north = temp;
                }
                (_, 180) => {
                    w_east *= -1;
                    w_north *= -1;
                }
                ('L', 270) | ('R', 90) => {
                    let temp = w_east;
                    w_east = w_north;
                    w_north = -temp;
                }
                _ => (),
            },
            'F' => {
                east += w_east * number;
                north += w_north * number;
            }
            _ => (),
        }
    }
    println!("{}", east.abs() + north.abs());
}
