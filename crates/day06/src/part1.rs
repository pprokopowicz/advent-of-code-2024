use crate::model::{Coordinates, Direction, Position};

pub fn solve(input: &Vec<Vec<Position>>) {
    let mut current_direction = Direction::North;
    let mut map = input.clone();
    let mut guard_cooridnates = guard_starting_coordinates(input);

    let map_height = map.len();
    let map_len = map[0].len();

    while !will_guard_leave_area(&guard_cooridnates, map_height, map_len, &current_direction) {
        let next_coordinates = next_coordinates(&guard_cooridnates, &current_direction);
        let next_position = map[next_coordinates.y][next_coordinates.x];

        match next_position {
            Position::Free(_) => {
                map[next_coordinates.y][next_coordinates.x] = Position::Free(true);
                guard_cooridnates = next_coordinates;
            }
            Position::Obstruction => current_direction = next_direction(&current_direction),
        }
    }

    println!("Part 1 solution: {}", result_sum(&map));
}

fn guard_starting_coordinates(input: &Vec<Vec<Position>>) -> Coordinates {
    input
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, position)| match position {
                    Position::Free(true) => Some(Coordinates { x, y }),
                    _ => None,
                })
        })
        .unwrap()
}

fn will_guard_leave_area(
    guard_cooridnates: &Coordinates,
    map_height: usize,
    map_len: usize,
    direction: &Direction,
) -> bool {
    match direction {
        Direction::North => guard_cooridnates.y == 0,
        Direction::East => guard_cooridnates.x == map_len,
        Direction::South => guard_cooridnates.y == map_height - 1,
        Direction::West => guard_cooridnates.x == 0,
    }
}

fn next_coordinates(guard_cooridnates: &Coordinates, direction: &Direction) -> Coordinates {
    match direction {
        Direction::North => Coordinates {
            x: guard_cooridnates.x,
            y: guard_cooridnates.y - 1,
        },
        Direction::East => Coordinates {
            x: guard_cooridnates.x + 1,
            y: guard_cooridnates.y,
        },
        Direction::South => Coordinates {
            x: guard_cooridnates.x,
            y: guard_cooridnates.y + 1,
        },
        Direction::West => Coordinates {
            x: guard_cooridnates.x - 1,
            y: guard_cooridnates.y,
        },
    }
}

fn next_direction(current_direction: &Direction) -> Direction {
    match current_direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
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
