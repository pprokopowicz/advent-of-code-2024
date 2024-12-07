use std::collections::HashMap;

use crate::model::Instruction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Addition,
    Multiplication,
}

pub fn solve(input: &Vec<Instruction>) {
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

            let output = operators_list.iter().find_map(|operators| {
                let test_value = instruction.test_value;
                let calculated = calculate_numbers(&instruction.numbers, operators);

                if test_value == calculated {
                    Some(test_value)
                } else {
                    None
                }
            });

            output
        })
        .sum::<usize>();

    println!("Part 1 solution: {}", result);
}

fn calculate_numbers(numbers: &Vec<usize>, operators: &Vec<Operator>) -> usize {
    let mut output = numbers[0];

    for index in 0..operators.len() {
        let next_num = numbers[index + 1];
        let operator = operators[index];

        match operator {
            Operator::Addition => output += next_num,
            Operator::Multiplication => output *= next_num,
        }
    }

    output
}

fn generate_operators(len: usize) -> Vec<Vec<Operator>> {
    let mut output = vec![vec![Operator::Addition], vec![Operator::Multiplication]];

    for _ in 1..len {
        let mut cloned = output.clone();

        output
            .iter_mut()
            .for_each(|operators| operators.push(Operator::Addition));
        cloned
            .iter_mut()
            .for_each(|operators| operators.push(Operator::Multiplication));

        output.append(&mut cloned);
    }

    output
}
