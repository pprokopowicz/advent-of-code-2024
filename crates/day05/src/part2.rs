use crate::parser::{Data, Rule, Updates};

pub fn solve(input: &Data) {
    let rules = &input.rules;
    let mut updates = input.updates.clone();

    updates.retain(|update| !is_update_correct(update, &rules.iter().collect::<Vec<&Rule>>()));

    updates.iter_mut().for_each(|updates| {
        let aplicable_rules = rules
            .iter()
            .filter(|rule| does_rule_apply(rule, updates))
            .collect::<Vec<&Rule>>();

        while !is_update_correct(updates, &aplicable_rules) {
            aplicable_rules.iter().for_each(|rule| {
                let lhs_index = updates
                    .list
                    .iter()
                    .position(|value| value == &rule.lhs)
                    .unwrap();
                let rhs_index = updates
                    .list
                    .iter()
                    .position(|value| value == &rule.rhs)
                    .unwrap();

                if lhs_index >= rhs_index {
                    let element = updates.list.remove(rhs_index);
                    updates.list.insert(lhs_index, element);
                }
            });
        }
    });

    let result = updates
        .iter()
        .map(|update| {
            let middle_index = update.list.len() / 2;
            update.list[middle_index]
        })
        .sum::<usize>();

    println!("Part 2 solution: {}", result);
}

fn is_update_correct(updates: &Updates, rules: &Vec<&Rule>) -> bool {
    rules.iter().all(|rule| {
        if does_rule_apply(rule, updates) {
            let lhs_index = updates
                .list
                .iter()
                .position(|value| value == &rule.lhs)
                .unwrap();
            let rhs_index = updates
                .list
                .iter()
                .position(|value| value == &rule.rhs)
                .unwrap();

            lhs_index < rhs_index
        } else {
            true
        }
    })
}

fn does_rule_apply(rule: &Rule, updates: &Updates) -> bool {
    updates.list.contains(&rule.lhs) && updates.list.contains(&rule.rhs)
}
