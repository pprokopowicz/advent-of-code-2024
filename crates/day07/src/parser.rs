use reader::{read_file, File};

use crate::model::Instruction;

pub fn parse() -> Vec<Instruction> {
    let content = read_file(&File::Day07);

    let output = content
        .lines()
        .map(|line| {
            let mut split = line.split(": ");

            let test_value = split
                .next()
                .expect("Split must have first part!")
                .parse::<usize>()
                .expect("First part of split must be a number!");
            let numbers = split
                .next()
                .expect("Split must have second part!")
                .split(" ")
                .map(|str_num| {
                    str_num
                        .parse::<usize>()
                        .expect("List must containt only numbers!")
                })
                .collect::<Vec<usize>>();

            Instruction {
                test_value,
                numbers,
            }
        })
        .collect::<Vec<Instruction>>();

    output
}
