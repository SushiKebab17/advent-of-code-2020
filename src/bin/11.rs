use array2d::Array2D;

fn main() {
    let input = advent_of_code_2020::input::input("11");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut array = parse(input);
    let mut switches = Vec::new();
    loop {
        for pos in array.positions() {
            let surroundings = get_surroundings_part1(&array, pos);
            if array[pos] == 'L' {
                if surroundings[1] == 0 {
                    switches.push(pos);
                }
            } else if array[pos] == '#' {
                if surroundings[1] >= 4 {
                    switches.push(pos);
                }
            }
        }
        if switches.is_empty() {
            break;
        }
        for &pos in &switches {
            if array[pos] == 'L' {
                array[pos] = '#';
            } else {
                array[pos] = 'L';
            }
        }
        switches.clear();
    }
    println!("{}", array.into_iter().filter(|x| *x == '#').count());
}

fn part_two(input: &[String]) {
    let mut array = parse(input);
    let mut switches = Vec::new();
    loop {
        for pos in array.positions() {
            let surroundings = get_surroundings_part2(&array, pos);
            if array[pos] == 'L' {
                if surroundings[1] == 0 {
                    switches.push(pos);
                }
            } else if array[pos] == '#' {
                if surroundings[1] >= 5 {
                    switches.push(pos);
                }
            }
        }
        if switches.is_empty() {
            break;
        }
        for &pos in &switches {
            if array[pos] == 'L' {
                array[pos] = '#';
            } else {
                array[pos] = 'L';
            }
        }
        switches.clear();
    }
    println!("{}", array.into_iter().filter(|x| *x == '#').count());
}

fn parse(input: &[String]) -> Array2D<char> {
    let mut array = Array2D::new(input[0].len(), input.len(), ' ');
    let mut pos = array.positions();
    for line in input {
        for c in line.chars() {
            array[pos.next().unwrap()] = c;
        }
    }
    array
}

fn get_surroundings_part1(array: &Array2D<char>, pos: (usize, usize)) -> [u32; 2] {
    let mut surroundings = [0 as u32; 2];
    if pos.0 > 0 && pos.1 > 0 {
        if let Some(item) = array.get((pos.0 - 1, pos.1 - 1)) {
            match item {
                'L' => surroundings[0] += 1,
                '#' => surroundings[1] += 1,
                _ => (),
            }
        }
    }
    if pos.1 > 0 {
        if let Some(item) = array.get((pos.0, pos.1 - 1)) {
            match item {
                'L' => surroundings[0] += 1,
                '#' => surroundings[1] += 1,
                _ => (),
            }
        }
    }
    if pos.0 < array.width() - 1 && pos.1 > 0 {
        if let Some(item) = array.get((pos.0 + 1, pos.1 - 1)) {
            match item {
                'L' => surroundings[0] += 1,
                '#' => surroundings[1] += 1,
                _ => (),
            }
        }
    }
    if pos.0 > 0 {
        if let Some(item) = array.get((pos.0 - 1, pos.1)) {
            match item {
                'L' => surroundings[0] += 1,
                '#' => surroundings[1] += 1,
                _ => (),
            }
        }
    }
    if pos.0 < array.width() - 1 {
        if let Some(item) = array.get((pos.0 + 1, pos.1)) {
            match item {
                'L' => surroundings[0] += 1,
                '#' => surroundings[1] += 1,
                _ => (),
            }
        }
    }
    if pos.0 > 0 && pos.1 < array.height() - 1 {
        if let Some(item) = array.get((pos.0 - 1, pos.1 + 1)) {
            match item {
                'L' => surroundings[0] += 1,
                '#' => surroundings[1] += 1,
                _ => (),
            }
        }
    }
    if pos.1 < array.height() - 1 {
        if let Some(item) = array.get((pos.0, pos.1 + 1)) {
            match item {
                'L' => surroundings[0] += 1,
                '#' => surroundings[1] += 1,
                _ => (),
            }
        }
    }
    if pos.0 < array.width() - 1 && pos.1 < array.height() - 1 {
        if let Some(item) = array.get((pos.0 + 1, pos.1 + 1)) {
            match item {
                'L' => surroundings[0] += 1,
                '#' => surroundings[1] += 1,
                _ => (),
            }
        }
    }
    surroundings
}

fn get_surroundings_part2(array: &Array2D<char>, pos: (usize, usize)) -> [u32; 2] {
    let mut surroundings = [0 as u32; 2];
    let mut x = 1;
    while pos.0 >= x && pos.1 >= x {
        if let Some(item) = array.get((pos.0 - x, pos.1 - x)) {
            match item {
                'L' => {
                    surroundings[0] += 1;
                    break;
                }
                '#' => {
                    surroundings[1] += 1;
                    break;
                }
                _ => x += 1,
            }
        }
    }
    x = 1;
    while pos.1 >= x {
        if let Some(item) = array.get((pos.0, pos.1 - x)) {
            match item {
                'L' => {
                    surroundings[0] += 1;
                    break;
                }
                '#' => {
                    surroundings[1] += 1;
                    break;
                }
                _ => x += 1,
            }
        }
    }
    x = 1;
    while pos.0 + x < array.width() && pos.1 >= x {
        if let Some(item) = array.get((pos.0 + x, pos.1 - x)) {
            match item {
                'L' => {
                    surroundings[0] += 1;
                    break;
                }
                '#' => {
                    surroundings[1] += 1;
                    break;
                }
                _ => x += 1,
            }
        }
    }
    x = 1;
    while pos.0 >= x {
        if let Some(item) = array.get((pos.0 - x, pos.1)) {
            match item {
                'L' => {
                    surroundings[0] += 1;
                    break;
                }
                '#' => {
                    surroundings[1] += 1;
                    break;
                }
                _ => x += 1,
            }
        }
    }
    x = 1;
    while pos.0 + x < array.width() {
        if let Some(item) = array.get((pos.0 + x, pos.1)) {
            match item {
                'L' => {
                    surroundings[0] += 1;
                    break;
                }
                '#' => {
                    surroundings[1] += 1;
                    break;
                }
                _ => x += 1,
            }
        }
    }
    x = 1;
    while pos.0 >= x && pos.1 + x < array.height() {
        if let Some(item) = array.get((pos.0 - x, pos.1 + x)) {
            match item {
                'L' => {
                    surroundings[0] += 1;
                    break;
                }
                '#' => {
                    surroundings[1] += 1;
                    break;
                }
                _ => x += 1,
            }
        }
    }
    x = 1;
    while pos.1 + x < array.height() {
        if let Some(item) = array.get((pos.0, pos.1 + x)) {
            match item {
                'L' => {
                    surroundings[0] += 1;
                    break;
                }
                '#' => {
                    surroundings[1] += 1;
                    break;
                }
                _ => x += 1,
            }
        }
    }
    x = 1;
    while pos.0 + x < array.width() && pos.1 + x < array.height() {
        if let Some(item) = array.get((pos.0 + x, pos.1 + x)) {
            match item {
                'L' => {
                    surroundings[0] += 1;
                    break;
                }
                '#' => {
                    surroundings[1] += 1;
                    break;
                }
                _ => x += 1,
            }
        }
    }
    surroundings
}
