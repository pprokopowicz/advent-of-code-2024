use crate::model::{Coordinates, Position};
use std::collections::{HashMap, HashSet};

pub fn solve(input: &Vec<Vec<Position>>) {
    let mut result = HashSet::new();
    let frequency_map = frequency_map(input);

    let max_height = input.len() as isize;
    let max_len = input[0].len() as isize;

    frequency_map.into_values().for_each(|coordinate_list| {
        for index in 0..coordinate_list.len() {
            let lhs_coordinate = coordinate_list[index];

            for outer in (index + 1)..coordinate_list.len() {
                let rhs_coordinate = coordinate_list[outer];

                let x_diff = lhs_coordinate.x.abs_diff(rhs_coordinate.x) as isize;
                let y_diff = lhs_coordinate.y.abs_diff(rhs_coordinate.y) as isize;

                let (first_x, second_x) = if lhs_coordinate.x > rhs_coordinate.x {
                    (lhs_coordinate.x + x_diff, rhs_coordinate.x - x_diff)
                } else {
                    (lhs_coordinate.x - x_diff, rhs_coordinate.x + x_diff)
                };

                let (first_y, second_y) = if lhs_coordinate.y > rhs_coordinate.y {
                    (lhs_coordinate.y + y_diff, rhs_coordinate.y - y_diff)
                } else {
                    (lhs_coordinate.y - y_diff, rhs_coordinate.y + y_diff)
                };

                append(first_x, first_y, max_len, max_height, &mut result);
                append(second_x, second_y, max_len, max_height, &mut result);
            }
        }
    });

    println!("Part 1 solution: {}", result.len());
}

fn append(x: isize, y: isize, max_len: isize, max_height: isize, set: &mut HashSet<Coordinates>) {
    if x >= 0 && x < max_len && y >= 0 && y < max_height {
        let node = Coordinates { x, y };
        set.insert(node);
    }
}

fn frequency_map(input: &Vec<Vec<Position>>) -> HashMap<char, Vec<Coordinates>> {
    let mut frequency_map = HashMap::new();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            match input[y][x] {
                Position::Antenna(frequency) => {
                    frequency_map
                        .entry(frequency)
                        .and_modify(|vec: &mut Vec<Coordinates>| {
                            vec.push(Coordinates {
                                x: x as isize,
                                y: y as isize,
                            })
                        })
                        .or_insert(vec![Coordinates {
                            x: x as isize,
                            y: y as isize,
                        }]);
                }
                Position::Free => {}
            }
        }
    }

    frequency_map
}
