use std::collections::{HashMap, HashSet};

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
    println!("{}", active_cubes.len());
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

fn amend_space(active_cubes: &mut HashSet<(isize, isize, isize)>) {
    let mut neighbours_map = HashMap::new();
    for coord in active_cubes.iter() {
        for neighbour in get_neighbours(coord) {
            neighbours_map
                .entry(neighbour)
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }
    }
    let mut new_actives: HashSet<(isize, isize, isize)> = HashSet::new();
    for coord in neighbours_map.keys() {
        if active_cubes.contains(coord)
            && (neighbours_map[coord] == 2 || neighbours_map[coord] == 3)
        {
            new_actives.insert(*coord);
        } else if !active_cubes.contains(coord) && neighbours_map[coord] == 3 {
            new_actives.insert(*coord);
        }
    }
    *active_cubes = new_actives;
}

fn get_neighbours(coord: &(isize, isize, isize)) -> [(isize, isize, isize); 26] {
    let mut neighbours = [(0, 0, 0); 26];
    let mut i = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if (x, y, z) != (0, 0, 0) {
                    neighbours[i] = (coord.0 + x, coord.1 + y, coord.2 + z);
                    i += 1;
                }
            }
        }
    }
    neighbours
}
