use rayon::prelude::*;
use reader::{read_file, File};

use crate::model::Position;

pub fn parse() -> Vec<Vec<Position>> {
    let content = read_file(&File::Day06);

    let output = content
        .par_lines()
        .map(|line| {
            line.chars()
                .map(|char_pos| match char_pos {
                    '.' => Position::Free(false),
                    '#' => Position::Obstruction,
                    '^' => Position::Free(true),
                    _ => panic!("Unknown character {}!", char_pos),
                })
                .collect()
        })
        .collect();

    output
}
