use crate::model::{PageUpdates, Rule};

pub fn indexes_from(rule: &Rule, updates: &PageUpdates) -> (usize, usize) {
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

    (lhs_index, rhs_index)
}

pub fn does_rule_apply(rule: &Rule, updates: &PageUpdates) -> bool {
    updates.list.contains(&rule.lhs) && updates.list.contains(&rule.rhs)
}

pub fn sum_from_updates_middle_element(updates: &Vec<PageUpdates>) -> usize {
    updates
        .iter()
        .map(|page_updates| {
            let middle_index = page_updates.list.len() / 2;
            page_updates.list[middle_index]
        })
        .sum::<usize>()
}
