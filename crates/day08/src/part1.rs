use crate::{
    helper,
    model::{Coordinates, Position},
};
use std::collections::HashSet;

pub fn solve(input: &Vec<Vec<Position>>) {
    let mut result = HashSet::new();
    let frequency_map = helper::frequency_map(input);

    let max_height = input.len() as isize;
    let max_len = input[0].len() as isize;

    frequency_map.into_values().for_each(|coordinate_list| {
        for index in 0..coordinate_list.len() {
            let lhs_coordinate = coordinate_list[index];

            for outer in (index + 1)..coordinate_list.len() {
                let rhs_coordinate = coordinate_list[outer];

                let (lhs_x_diff, rhs_x_diff) =
                    helper::difference(lhs_coordinate.x, rhs_coordinate.x);
                let (lhs_y_diff, rhs_y_diff) =
                    helper::difference(lhs_coordinate.y, rhs_coordinate.y);

                let lhs_x = lhs_coordinate.x + lhs_x_diff;
                let lhs_y = lhs_coordinate.y + lhs_y_diff;
                let rhs_x = rhs_coordinate.x + rhs_x_diff;
                let rhs_y = rhs_coordinate.y + rhs_y_diff;

                append(lhs_x, lhs_y, max_len, max_height, &mut result);
                append(rhs_x, rhs_y, max_len, max_height, &mut result);
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
