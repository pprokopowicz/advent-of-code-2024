pub fn solve(input: &(Vec<usize>, Vec<usize>)) {
    let mut first = input.0.clone();
    first.sort();

    let mut second = input.1.clone();
    second.sort();

    let result = first
        .iter()
        .zip(second)
        .map(|(first, second)| first.abs_diff(second))
        .sum::<usize>();

    println!("Part 1 solution: {}", result);
}
