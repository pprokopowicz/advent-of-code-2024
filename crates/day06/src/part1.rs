use crate::{
    helper,
    model::{Direction, Position},
};

pub fn solve(input: &Vec<Vec<Position>>) {
    let mut current_direction = Direction::North;
    let mut map = input.clone();
    let mut guard_cooridnates = helper::guard_starting_coordinates(input);

    let map_height = map.len();
    let map_len = map[0].len();

    while !helper::will_guard_leave_area(
        &guard_cooridnates,
        map_height,
        map_len,
        &current_direction,
    ) {
        let next_coordinates = helper::next_coordinates(&guard_cooridnates, &current_direction);
        let next_position = map[next_coordinates.y][next_coordinates.x];

        match next_position {
            Position::Free(_) => {
                map[next_coordinates.y][next_coordinates.x] = Position::Free(true);
                guard_cooridnates = next_coordinates;
            }
            Position::Obstruction => current_direction = helper::next_direction(&current_direction),
        }
    }

    println!("Part 1 solution: {}", result_sum(&map));
}

fn result_sum(map: &Vec<Vec<Position>>) -> usize {
    map.iter()
        .map(|line| {
            line.iter()
                .map(|position| match position {
                    Position::Free(true) => 1,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}
