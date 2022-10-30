use array2d::Array2D;

fn main() {
    let input = advent_of_code_2020::input::input("03");
    part_one(&input);
    part_two(&input);
}

fn make_array(input: &[String]) -> Array2D<char> {
    let mut arr = Array2D::new(input[0].len(), input.len(), ' ');
    let mut pos = arr.positions();
    for line in input {
        for c in line.chars() {
            arr[pos.next().unwrap()] = c;
        }
    }
    arr
}

fn part_one(input: &[String]) {
    let array = make_array(input);
    println!("{}", count_trees(&array, 3, 1));
}

fn part_two(input: &[String]) {
    let array = make_array(input);
    let product = count_trees(&array, 1, 1)
        * count_trees(&array, 3, 1)
        * count_trees(&array, 5, 1)
        * count_trees(&array, 7, 1)
        * count_trees(&array, 1, 2);
    println!("{}", product);
}

fn count_trees(array: &Array2D<char>, x_change: usize, y_change: usize) -> usize {
    let (mut x, mut y) = (x_change, y_change);
    let mut count = 0;
    while y < array.height() {
        if array[[x, y]] == '#' {
            count += 1;
        }
        x = (x + x_change) % array.width();
        y += y_change;
    }
    count
}
