use std::collections::{HashMap, HashSet};

fn main() {
    let input = advent_of_code_2020::input::input("17");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let mut active_cubes = parse(input);
    for _ in 0..6 {
        amend_space(&mut active_cubes, &[0]);
    }
    println!("{}", active_cubes.len());
}

fn part_two(input: &[String]) {
    let mut active_cubes = parse(input);
    for _ in 0..6 {
        amend_space(&mut active_cubes, &[-1, 0, 1]);
    }
    println!("{}", active_cubes.len());
}

fn parse(input: &[String]) -> HashSet<(i32, i32, i32, i32)> {
    let mut active_cubes = HashSet::new();
    for (line_num, line) in input.iter().enumerate() {
        for (c_num, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert((line_num as i32, c_num as i32, 0, 0));
            }
        }
    }
    active_cubes
}

fn amend_space(active_cubes: &mut HashSet<(i32, i32, i32, i32)>, range: &[i32]) {
    let mut neighbours_map = HashMap::new();
    for coord in active_cubes.iter() {
        for neighbour in get_neighbours(coord, range) {
            neighbours_map
                .entry(neighbour)
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }
    }
    let mut new_actives: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    for (coord, neighbours) in neighbours_map {
        if active_cubes.contains(&coord) && (neighbours == 2 || neighbours == 3) {
            new_actives.insert(coord);
        } else if neighbours == 3 {
            new_actives.insert(coord);
        }
    }
    *active_cubes = new_actives;
}

fn get_neighbours(coord: &(i32, i32, i32, i32), range: &[i32]) -> Vec<(i32, i32, i32, i32)> {
    let mut neighbours = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for &w in range {
                    if (x, y, z, w) != (0, 0, 0, 0) {
                        neighbours.push((coord.0 + x, coord.1 + y, coord.2 + z, coord.3 + w));
                    }
                }
            }
        }
    }
    neighbours
}
