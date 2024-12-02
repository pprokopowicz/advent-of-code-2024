mod parser;
mod solver;

fn main() {
    let input = parser::parse();

    let part1 = solver::solve(&input, 0);
    let part2 = solver::solve(&input, 1);

    println!("Part 1 solution: {}", part1);
    println!("Part 2 solution: {}", part2);
}
