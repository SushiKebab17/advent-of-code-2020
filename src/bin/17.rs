use std::collections::HashSet;

use array2d::Array2D;

fn main() {
    let input = advent_of_code_2020::input::input("17");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut active_cubes = parse(input);
    for _ in 0..6 {
        amend_space(&mut active_cubes);
    }
}

fn part_two(_input: &[String]) {}

fn parse(input: &[String]) -> HashSet<(isize, isize, isize)> {
    let mut active_cubes = HashSet::new();
    for (line_num, line) in input.iter().enumerate() {
        for (c_num, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert((line_num as isize, c_num as isize, 0));
            }
        }
    }
    active_cubes
}

fn amend_space(space: &mut HashSet<(isize, isize, isize)>) {}
