pub fn solve(input: &Vec<Vec<char>>) {
    // let max_height = input.len();

    let mut result = 0;

    for y in 0..(input.len() - 2) {
        let row = &input[y];

        for x in 0..(row.len() - 2) {
            if check_right_down(input, x, y) && check_left_down(input, x, y) {
                result += 1;
            }
        }
    }

    println!("Part 2 solution: {}", result);
}

fn check_right_down(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    (input[y][x] == 'M' && input[y + 1][x + 1] == 'A' && input[y + 2][x + 2] == 'S')
        || (input[y][x] == 'S' && input[y + 1][x + 1] == 'A' && input[y + 2][x + 2] == 'M')
}

fn check_left_down(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    (input[y][x + 2] == 'M' && input[y + 1][x + 1] == 'A' && input[y + 2][x] == 'S')
        || (input[y][x + 2] == 'S' && input[y + 1][x + 1] == 'A' && input[y + 2][x] == 'M')
}
