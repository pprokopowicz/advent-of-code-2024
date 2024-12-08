use std::collections::HashMap;

use crate::model::{Coordinates, Position};

pub fn frequency_map(input: &Vec<Vec<Position>>) -> HashMap<char, Vec<Coordinates>> {
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
