use crate::{
    helper,
    model::{Instruction, Operator},
};

pub fn solve(input: &Vec<Instruction>) {
    let result = helper::instruction_sum(input, &generate_operators);

    println!("Part 2 solution: {}", result);
}

fn generate_operators(len: usize) -> Vec<Vec<Operator>> {
    let mut output = vec![
        vec![Operator::Addition],
        vec![Operator::Multiplication],
        vec![Operator::Concatenation],
    ];

    for _ in 1..len {
        let mut cloned_multiplication = output.clone();
        let mut cloned_concatenation = output.clone();

        output
            .iter_mut()
            .for_each(|operators| operators.push(Operator::Addition));
        cloned_multiplication
            .iter_mut()
            .for_each(|operators| operators.push(Operator::Multiplication));
        cloned_concatenation
            .iter_mut()
            .for_each(|operators| operators.push(Operator::Concatenation));

        output.append(&mut cloned_multiplication);
        output.append(&mut cloned_concatenation);
    }

    output
}
