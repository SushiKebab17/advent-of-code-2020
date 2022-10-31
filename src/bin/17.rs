use std::collections::HashSet;

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

fn parse(input: &[String]) -> Array2D<char> {
    let mut arr = Array2D::new(input[0].len(), input.len(), ' ');
    let mut pos = arr.positions();
    for line in input {
        for c in line.chars() {
            arr[pos.next().unwrap()] = c;
        }
    }
    arr
}

fn amend_space(space: &mut [Array2D<char>]) {
    let mut new_space = Vec::new();
    let first = &space[0];
    for z in 0..=2 * space.len() {
        new_space.push(Array2D::new(first.width() + 2, first.height() + 2, '.'));
    }
    for plane in space {}
}
