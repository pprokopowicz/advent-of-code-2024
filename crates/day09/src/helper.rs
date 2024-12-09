use crate::model::{ContentType, DiskContent};

pub fn checksum(input: &Vec<DiskContent>) -> usize {
    let mut length_sum = 0;
    let mut sum = 0;

    for index in 0..input.len() {
        match input[index].content_type {
            ContentType::File(id) => {
                for inner in 0..input[index].length {
                    sum += (inner + length_sum) * id;
                }
            }
            ContentType::Free => {}
        }

        length_sum += input[index].length
    }

    sum
}
