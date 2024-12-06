// Save start coordinates of already visited position, if you land there again and direction or next direction is the same it's a loop

use crate::{
    helper,
    model::{Coordinates, Direction, Position},
};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Output {
    Exit,
    Loop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoopStart {
    coordinates: Coordinates,
    direction: Direction,
}

pub fn solve(input: &Vec<Vec<Position>>) {
    let result = input
        .par_iter()
        .enumerate()
        .map(|(y, _)| {
            input[y]
                .par_iter()
                .enumerate()
                .map(|(x, position)| match position {
                    Position::Free(false) => {
                        let mut modified_map = input.clone();
                        modified_map[y][x] = Position::Obstruction;

                        let sim_output = map_sim_output(&modified_map);

                        match sim_output {
                            Output::Loop => 1,
                            Output::Exit => 0,
                        }
                    }
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Part 2 solution: {}", result);
}

fn map_sim_output(input: &Vec<Vec<Position>>) -> Output {
    let mut current_direction = Direction::North;
    let mut map = input.clone();
    let mut guard_cooridnates = helper::guard_starting_coordinates(input);

    let mut loop_start: Option<LoopStart> = None;

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
            Position::Free(false) => {
                loop_start = None;
                map[next_coordinates.y][next_coordinates.x] = Position::Free(true);
                guard_cooridnates = next_coordinates;
            }
            Position::Free(true) => {
                map[next_coordinates.y][next_coordinates.x] = Position::Free(true);
                guard_cooridnates = next_coordinates;

                match loop_start {
                    Some(previous_start) => {
                        if previous_start.coordinates == guard_cooridnates
                            && previous_start.direction == current_direction
                        {
                            return Output::Loop;
                        }
                    }
                    None => {
                        loop_start = Some(LoopStart {
                            coordinates: next_coordinates.clone(),
                            direction: current_direction.clone(),
                        })
                    }
                };
            }
            Position::Obstruction => current_direction = helper::next_direction(&current_direction),
        }
    }

    Output::Exit
}
