use crate::{helper, model::ManualUpdateData};

pub fn solve(input: &ManualUpdateData) {
    let rules = &input.rules;
    let mut updates = input.updates.clone();

    rules.iter().for_each(|rule| {
        updates.retain(|page_updates| {
            if helper::does_rule_apply(rule, page_updates) {
                let (lhs_index, rhs_index) = helper::indexes_from(rule, page_updates);

                lhs_index < rhs_index
            } else {
                true
            }
        });
    });

    let result = helper::sum_from_updates_middle_element(&updates);

    println!("Part 1 solution: {}", result);
}
