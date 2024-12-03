use reader::{read_file, File};
use regex::Regex;

#[derive(Debug)]
pub enum Instruction {
    Do,
    Dont,
    Mul(usize, usize),
}

pub fn parse() -> Vec<Instruction> {
    let content = read_file(File::Day03);
    let regex = Regex::new(r"mul\(\d*,\d*\)|do\(\)|don't\(\)").unwrap();

    let output = regex
        .captures_iter(&content)
        .map(|capture| {
            let extracted: (&str, [&str; 0]) = capture.extract();
            let instruction = extracted.0;

            match instruction {
                "do()" => Instruction::Do,
                "don't()" => Instruction::Dont,
                _ => mul_instruction(instruction),
            }
        })
        .collect::<Vec<Instruction>>();

    output
}

fn mul_instruction(value: &str) -> Instruction {
    let numbers = value
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

    Instruction::Mul(first_number, second_number)
}
