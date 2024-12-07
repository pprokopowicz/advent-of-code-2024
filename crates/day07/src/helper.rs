use crate::model::{Instruction, Operator};
use std::collections::HashMap;

pub fn instruction_sum(
    input: &Vec<Instruction>,
    generate_operators: &dyn Fn(usize) -> Vec<Vec<Operator>>,
) -> usize {
    let mut operators_cache = HashMap::new();

    let result = input
        .iter()
        .filter_map(|instruction| {
            let operator_len = instruction.numbers.len() - 1;
            let operators_list = match operators_cache.get(&operator_len) {
                Some(operators) => operators,
                None => {
                    let operators = generate_operators(operator_len);
                    operators_cache.insert(operator_len, operators);
                    operators_cache.get(&operator_len).unwrap()
                }
            };

            calculate_instruction(instruction, operators_list)
        })
        .sum::<usize>();

    result
}

fn calculate_numbers(numbers: &Vec<usize>, operators: &Vec<Operator>) -> usize {
    let mut output = numbers[0];

    for index in 0..operators.len() {
        let next_num = numbers[index + 1];
        let operator = operators[index];

        match operator {
            Operator::Addition => output += next_num,
            Operator::Multiplication => output *= next_num,
            Operator::Concatenation => output = concatinate(output, next_num),
        }
    }

    output
}

fn concatinate(lhs: usize, rhs: usize) -> usize {
    lhs as usize * 10usize.pow(rhs.ilog10() + 1) + rhs as usize
}

fn calculate_instruction(
    instruction: &Instruction,
    operators_list: &Vec<Vec<Operator>>,
) -> Option<usize> {
    operators_list.iter().find_map(|operators| {
        let test_value = instruction.test_value;
        let calculated = calculate_numbers(&instruction.numbers, operators);

        if test_value == calculated {
            Some(test_value)
        } else {
            None
        }
    })
}
