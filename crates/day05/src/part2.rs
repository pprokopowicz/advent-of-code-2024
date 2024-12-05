use crate::{
    helper,
    model::{ManualUpdateData, PageUpdates, Rule},
};

pub fn solve(input: &ManualUpdateData) {
    let rules = &input.rules;
    let mut updates = input.updates.clone();

    updates.retain(|page_updates| {
        !is_update_correct(page_updates, &rules.iter().collect::<Vec<&Rule>>())
    });

    updates.iter_mut().for_each(|page_updates| {
        let aplicable_rules = applicable_rules_from(rules, page_updates);

        while !is_update_correct(page_updates, &aplicable_rules) {
            aplicable_rules.iter().for_each(|rule| {
                let (lhs_index, rhs_index) = helper::indexes_from(rule, page_updates);

                if lhs_index >= rhs_index {
                    let element = page_updates.list.remove(rhs_index);
                    page_updates.list.insert(lhs_index, element);
                }
            });
        }
    });

    let result = helper::sum_from_updates_middle_element(&updates);

    println!("Part 2 solution: {}", result);
}

fn is_update_correct(updates: &PageUpdates, rules: &Vec<&Rule>) -> bool {
    rules.iter().all(|rule| {
        if helper::does_rule_apply(rule, updates) {
            let (lhs_index, rhs_index) = helper::indexes_from(rule, updates);

            lhs_index < rhs_index
        } else {
            true
        }
    })
}

fn applicable_rules_from<'a>(rules: &'a Vec<Rule>, page_updates: &PageUpdates) -> Vec<&'a Rule> {
    rules
        .iter()
        .filter(|rule| helper::does_rule_apply(rule, page_updates))
        .collect::<Vec<&Rule>>()
}
