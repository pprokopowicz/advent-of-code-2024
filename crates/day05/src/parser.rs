use reader::{read_file, File};

use crate::model::{ManualUpdateData, PageUpdates, Rule};

pub fn parse() -> ManualUpdateData {
    let content = read_file(&File::Day05);

    let mut split = content.split("\n\n");
    let rules_str = split.next().expect("Split must containt first part!");
    let updates_str = split.next().expect("Split must containt second part!");

    let rules = rules_from(rules_str);
    let updates = updates_from(updates_str);

    ManualUpdateData { rules, updates }
}

fn rules_from(rules_str: &str) -> Vec<Rule> {
    rules_str
        .lines()
        .map(|line| {
            let mut split = line.split("|");

            let lhs = split
                .next()
                .expect("Rule split must contain first part!")
                .parse::<usize>()
                .expect("First part of rule split must be a number!");
            let rhs = split
                .next()
                .expect("Rule split must contain second part!")
                .parse::<usize>()
                .expect("Second part of rule split must be a number!");

            Rule { lhs, rhs }
        })
        .collect::<Vec<Rule>>()
}

fn updates_from(updates_str: &str) -> Vec<PageUpdates> {
    updates_str
        .lines()
        .map(|line| {
            let update_list = line
                .split(",")
                .map(|num_str| num_str.parse::<usize>().expect("Update must be a number!"))
                .collect::<Vec<usize>>();

            PageUpdates { list: update_list }
        })
        .collect::<Vec<PageUpdates>>()
}
