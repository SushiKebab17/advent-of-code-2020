fn main() {
    let input = advent_of_code_2020::input::input("05");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut seat_id = 0;

    for line in input {
        let id = calculate_id(line);
        if id > seat_id {
            seat_id = id;
        }
    }
    println!("{}", seat_id);
}

fn part_two(input: &[String]) {
    let mut seat_ids: Vec<u32> = Vec::new();

    for line in input {
        seat_ids.push(calculate_id(line));
    }
    seat_ids.sort();

    let mut prev_id = seat_ids.get(0).unwrap() - 1;
    let mut my_id = 0;
    for id in seat_ids {
        if id != prev_id + 1 {
            my_id = id - 1;
        }
        prev_id = id;
        println!("{}", prev_id);
    }
    println!("{}", my_id);
}

fn calculate_id(seat: &str) -> u32 {
    let row_partition = &seat[..=6];
    let mut min_row = 0;
    let mut max_row = 127;

    for c in row_partition.chars() {
        match c {
            'F' => max_row = (min_row + max_row) / 2,
            'B' => min_row = (min_row + max_row) / 2 + 1,
            _ => (),
        }
    }

    let col_partition = &seat[7..=9];
    let mut min_col = 0;
    let mut max_col = 7;

    for c in col_partition.chars() {
        match c {
            'L' => max_col = (min_col + max_col) / 2,
            'R' => min_col = (min_col + max_col) / 2 + 1,
            _ => (),
        }
    }

    min_row * 8 + min_col
}
