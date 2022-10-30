fn main() {
    let input = advent_of_code_2020::input::input("08");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[String]) {
    let instructions: Vec<Instruction> = input.iter().map(Instruction::new).collect();
    println!("{}", eval_acc(&instructions, -1).1);
}

fn part_two(input: &[String]) {
    let instructions: Vec<Instruction> = input.iter().map(Instruction::new).collect();

    for x in 0..instructions.len() {
        let (check, accumulator) = eval_acc(&instructions, x as isize);
        if check {
            println!("{}", accumulator);
        }
    }
}

fn eval_acc(instructions: &[Instruction], switch: isize) -> (bool, i32) {
    let mut i: isize = 0;
    let mut acc = 0;
    let mut read_line = vec![false; instructions.len()];
    loop {
        if read_line[i as usize] == true {
            return (false, acc);
        }
        read_line[i as usize] = true;
        let instruction = &instructions[i as usize];
        if instruction.oper == "acc" {
            acc += instruction.arg;
        } else if (instruction.oper == "jmp") ^ (i == switch) {
            i += instruction.arg as isize - 1;
        }
        i += 1;
        if i == instructions.len() as isize {
            return (true, acc);
        }
    }
}

struct Instruction<'a> {
    oper: &'a str,
    arg: i32,
}

impl<'a> Instruction<'a> {
    fn new(input: &'a String) -> Self {
        let mut split = input.split(" ");
        Self {
            oper: split.next().unwrap(),
            arg: split.next().unwrap().parse().unwrap(),
        }
    }
}
