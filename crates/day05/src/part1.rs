use crate::parser::Data;

pub fn solve(input: &Data) {
    let rules = &input.rules;
    let mut updates = input.updates.clone();

    rules.iter().for_each(|rule| {
        updates.retain(|update| {
            if update.list.contains(&rule.lhs) && update.list.contains(&rule.rhs) {
                let lhs_index = update
                    .list
                    .iter()
                    .position(|value| value == &rule.lhs)
                    .unwrap();
                let rhs_index = update
                    .list
                    .iter()
                    .position(|value| value == &rule.rhs)
                    .unwrap();

                lhs_index < rhs_index
            } else {
                true
            }
        });
    });

    let result = updates
        .iter()
        .map(|update| {
            let middle_index = update.list.len() / 2;
            update.list[middle_index]
        })
        .sum::<usize>();

    println!("Part 1 solution: {}", result);
}
