use array2d::Array2D;

fn main() {
    let input = advent_of_code_2020::input::input("17");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut space = Vec::new();
    space.push(parse(input));
    for _ in 0..6 {
        amend_space(&mut space);
    }
}

fn part_two(_input: &[String]) {}

fn parse(input: &[String]) -> Plane {
    let mut arr = Array2D::new(input[0].len(), input.len(), ' ');
    let mut pos = arr.positions();
    for line in input {
        for c in line.chars() {
            arr[pos.next().unwrap()] = c;
        }
    }
    Plane {
        z: (0),
        array: (arr),
    }
}

fn amend_space(space: &mut [Plane]) {}

struct Plane {
    z: isize,
    array: Array2D<char>,
}
