use crate::parser::Instruction;

pub fn solve(input: &Vec<Instruction>) {
    let result = input
        .iter()
        .filter_map(|instruction| match instruction {
            Instruction::Do => None,
            Instruction::Dont => None,
            Instruction::Mul(first_number, second_number) => Some(first_number * second_number),
        })
        .sum::<usize>();

    println!("Part 1 solution: {}", result);
}
