use std::collections::HashSet;

use crate::{
    helper,
    model::{Coordinates, Position},
};

pub fn solve(input: &Vec<Vec<Position>>) {
    let mut result = HashSet::new();
    let frequency_map = helper::frequency_map(input);

    let max_height = input.len() as isize;
    let max_len = input[0].len() as isize;

    frequency_map.into_values().for_each(|coordinate_list| {
        for index in 0..coordinate_list.len() {
            let lhs_coordinate = coordinate_list[index];

            let node = Coordinates {
                x: lhs_coordinate.x,
                y: lhs_coordinate.y,
            };
            result.insert(node);

            for outer in (index + 1)..coordinate_list.len() {
                let rhs_coordinate = coordinate_list[outer];

                let x_diff = lhs_coordinate.x.abs_diff(rhs_coordinate.x) as isize;
                let y_diff = lhs_coordinate.y.abs_diff(rhs_coordinate.y) as isize;

                let (lhs_x_diff, rhs_x_diff) = if lhs_coordinate.x > rhs_coordinate.x {
                    (x_diff, -x_diff)
                } else {
                    (-x_diff, x_diff)
                };

                let (lhs_y_diff, rhs_y_diff) = if lhs_coordinate.y > rhs_coordinate.y {
                    (y_diff, -y_diff)
                } else {
                    (-y_diff, y_diff)
                };

                insert_all_nodes(
                    lhs_coordinate.x,
                    lhs_coordinate.y,
                    lhs_x_diff,
                    lhs_y_diff,
                    max_len,
                    max_height,
                    &mut result,
                );
                insert_all_nodes(
                    rhs_coordinate.x,
                    rhs_coordinate.y,
                    rhs_x_diff,
                    rhs_y_diff,
                    max_len,
                    max_height,
                    &mut result,
                );
            }
        }
    });

    println!("Part 2 solution: {}", result.len());
}

fn insert_all_nodes(
    x: isize,
    y: isize,
    x_diff: isize,
    y_diff: isize,
    max_len: isize,
    max_height: isize,
    set: &mut HashSet<Coordinates>,
) {
    let mut x = x + x_diff;
    let mut y = y + y_diff;

    while x >= 0 && x < max_len && y >= 0 && y < max_height {
        let node = Coordinates { x: x, y: y };
        set.insert(node);

        x += x_diff;
        y += y_diff;
    }
}
