pub fn solve(input: &Vec<(usize, usize)>) {
    let result = input
        .iter()
        .map(|(first_number, second_number)| first_number * second_number)
        .sum::<usize>();

    println!("Part 1 solution: {}", result);
}
