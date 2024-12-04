use std::fs;

#[derive(Copy, Clone)]
pub enum File {
    Day01 = 1,
    Day02,
    Day03,
    Day04,
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

pub fn read_file(file: &File) -> String {
    match fs::read_to_string(file_name(&file)) {
        Ok(output) => output,
        Err(_) => fs::read_to_string(example_filename(&file)).unwrap(),
    }
}
