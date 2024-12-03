use reader::{read_file, File};
use regex::Regex;

pub fn parse() -> Vec<(usize, usize)> {
    let content = read_file(File::Day03);
    let regex = Regex::new(r"mul\(\d*,\d*\)").unwrap();

    let output = regex
        .captures_iter(&content)
        .map(|capture| {
            let extracted: (&str, [&str; 0]) = capture.extract();
            let operation = extracted.0;

            let numbers = operation
                .strip_prefix("mul(")
                .expect("Operation must have \"mul\" prefix!")
                .strip_suffix(")")
                .expect("Operation must have \")\" sufix!");

            let mut split = numbers.split(",");

            let first_number = split
                .next()
                .unwrap()
                .parse::<usize>()
                .expect("First part of operation must be a number!");
            let second_number = split
                .next()
                .unwrap()
                .parse::<usize>()
                .expect("Second part of operation must be a number!");

            (first_number, second_number)
        })
        .collect::<Vec<(usize, usize)>>();

    output
}
