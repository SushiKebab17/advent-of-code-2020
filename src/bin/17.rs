use std::collections::{HashSet, HashMap};

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

fn amend_space(space: &mut HashSet<(isize, isize, isize)>) {
    let mut neighbours_map = HashMap::new();
    let mut new_space: HashSet<(isize, isize, isize)> = HashSet::new();
    for coord in space.iter() {
        for neighbour in get_neighbours(coord) {
            neighbours_map.entry(neighbour).and_modify(|n| *n += 1).or_insert(1);
        }
    }
}

fn get_neighbours(coord: &(isize, isize, isize)) -> [(isize, isize, isize); 26]{
    let mut neighbours = [(0,0,0); 26];
    let mut i = 0;
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                if x != 1 && y != 1 && z != 1 {
                    neighbours[i] = (coord.0 + x - 1, coord.1 + y - 1, coord.2 + z - 1);
                    i += 1;
                }
            }
        }
    }
    neighbours
}