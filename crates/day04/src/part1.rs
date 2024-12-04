pub fn solve(input: &Vec<Vec<char>>) {
    let max_height = input.len();

    let mut result = 0;

    for (y, row) in input.iter().enumerate() {
        let can_go_down = y <= max_height - 4;
        let can_go_up = y >= 3;
        let max_length = row.len();

        for x in 0..row.len() {
            let can_go_right = x <= max_length - 4;
            let can_go_left = x >= 3;

            if can_go_right && check_right(row, x) {
                result += 1;
            }

            if can_go_right && can_go_down && check_right_down(input, x, y) {
                result += 1;
            }

            if can_go_down && check_down(input, x, y) {
                result += 1;
            }

            if can_go_left && can_go_down && check_left_down(input, x, y) {
                result += 1;
            }

            if can_go_left && check_left(row, x) {
                result += 1;
            }

            if can_go_left && can_go_up && check_left_up(input, x, y) {
                result += 1;
            }

            if can_go_up && check_up(input, x, y) {
                result += 1;
            }

            if can_go_right && can_go_up && check_right_up(input, x, y) {
                result += 1;
            }
        }
    }

    println!("Part 1 solution: {}", result);
}

fn check_right(row: &Vec<char>, index: usize) -> bool {
    row[index] == 'X' && row[index + 1] == 'M' && row[index + 2] == 'A' && row[index + 3] == 'S'
}

fn check_right_down(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    input[y][x] == 'X'
        && input[y + 1][x + 1] == 'M'
        && input[y + 2][x + 2] == 'A'
        && input[y + 3][x + 3] == 'S'
}

fn check_down(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    input[y][x] == 'X' && input[y + 1][x] == 'M' && input[y + 2][x] == 'A' && input[y + 3][x] == 'S'
}

fn check_left_down(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    input[y][x] == 'X'
        && input[y + 1][x - 1] == 'M'
        && input[y + 2][x - 2] == 'A'
        && input[y + 3][x - 3] == 'S'
}

fn check_left(row: &Vec<char>, index: usize) -> bool {
    row[index] == 'X' && row[index - 1] == 'M' && row[index - 2] == 'A' && row[index - 3] == 'S'
}

fn check_left_up(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    input[y][x] == 'X'
        && input[y - 1][x - 1] == 'M'
        && input[y - 2][x - 2] == 'A'
        && input[y - 3][x - 3] == 'S'
}

fn check_up(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    input[y][x] == 'X' && input[y - 1][x] == 'M' && input[y - 2][x] == 'A' && input[y - 3][x] == 'S'
}

fn check_right_up(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    input[y][x] == 'X'
        && input[y - 1][x + 1] == 'M'
        && input[y - 2][x + 2] == 'A'
        && input[y - 3][x + 3] == 'S'
}
