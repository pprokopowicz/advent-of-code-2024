use crate::{
    helper,
    model::{Instruction, Operator},
};

pub fn solve(input: &Vec<Instruction>) {
    let result = helper::instruction_sum(input, &generate_operators);

    println!("Part 1 solution: {}", result);
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
