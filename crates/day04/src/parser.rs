use reader::{read_file, File};

pub fn parse() -> Vec<Vec<char>> {
    let content = read_file(&File::Day04);

    let output = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    output
}
