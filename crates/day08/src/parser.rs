use crate::model::Position;
use reader::{read_file, File};

pub fn parse() -> Vec<Vec<Position>> {
    let content = read_file(&File::Day08);

    let output = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Position::Free,
                    _ => Position::Antenna(ch),
                })
                .collect::<Vec<Position>>()
        })
        .collect::<Vec<Vec<Position>>>();

    output
}
