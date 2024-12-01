use std::collections::HashMap;

pub fn solve(input: &(Vec<usize>, Vec<usize>)) {
    let first = input.0.clone();
    let second = input.1.clone();

    let map = second
        .iter()
        .fold(HashMap::<&usize, usize>::new(), |mut acc, number| {
            acc.entry(number).and_modify(|num| *num += 1).or_insert(1);
            acc
        });

    let result = first
        .iter()
        .map(|num| {
            let value = map.get(num).unwrap_or(&0);
            value * num
        })
        .sum::<usize>();

    println!("Part 2 solution: {}", result);
}
