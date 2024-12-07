use std::fs;

#[derive(Copy, Clone)]
pub enum File {
    Day01 = 1,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
}

pub fn read_file(file: &File) -> String {
    match fs::read_to_string(file_name(&file)) {
        Ok(output) => output,
        Err(_) => read_example_file(file),
    }
}

pub fn read_example_file(file: &File) -> String {
    fs::read_to_string(example_filename(&file)).unwrap()
}

fn file_name(file: &File) -> String {
    let index = *file as u32;

    if index > 9 {
        format!("inputs/day{}.input", index)
    } else {
        format!("inputs/day0{}.input", index)
    }
}

fn example_filename(file: &File) -> String {
    let index = *file as u32;

    if index > 9 {
        format!("examples/day{}.example", index)
    } else {
        format!("examples/day0{}.example", index)
    }
}
