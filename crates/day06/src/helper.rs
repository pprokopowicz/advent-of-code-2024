use crate::model::{Coordinates, Direction, Position};

pub fn guard_starting_coordinates(input: &Vec<Vec<Position>>) -> Coordinates {
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

pub fn will_guard_leave_area(
    guard_cooridnates: &Coordinates,
    map_height: usize,
    map_len: usize,
    direction: &Direction,
) -> bool {
    match direction {
        Direction::North => guard_cooridnates.y == 0,
        Direction::East => guard_cooridnates.x == map_len - 1,
        Direction::South => guard_cooridnates.y == map_height - 1,
        Direction::West => guard_cooridnates.x == 0,
    }
}

pub fn next_coordinates(guard_cooridnates: &Coordinates, direction: &Direction) -> Coordinates {
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

pub fn next_direction(current_direction: &Direction) -> Direction {
    match current_direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}
