use reader::{read_file, File};

pub fn parse() -> Vec<Vec<usize>> {
    let content = read_file(&File::Day02);

    let output = content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|str_num| {
                    str_num
                        .parse::<usize>()
                        .expect(&format!("{} is not a number", str_num))
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    output
}
