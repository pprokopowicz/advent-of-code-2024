use crate::parser::Instruction;

pub fn solve(input: &Vec<Instruction>) {
    let mut is_mul_enabled = true;
    let mut result = 0;

    for instruction in input {
        match instruction {
            Instruction::Do => is_mul_enabled = true,
            Instruction::Dont => is_mul_enabled = false,
            Instruction::Mul(first_number, second_number) => {
                if is_mul_enabled {
                    result += first_number * second_number;
                }
            }
        }
    }

    println!("Part 2 solution: {}", result);
}
