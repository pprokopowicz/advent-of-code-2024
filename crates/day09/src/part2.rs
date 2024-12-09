use crate::{
    helper,
    model::{ContentType, DiskContent},
};

pub fn solve(input: &Vec<DiskContent>) {
    let mut input = input.clone();
    let mut reverse = input.clone();
    reverse.reverse();

    for reverse_index in 0..reverse.len() {
        let last = reverse[reverse_index];

        match last.content_type {
            ContentType::File(_) => {
                let last_index = index_of_disk_content(last, &input);

                let first_free_index = first_free_index(last_index, last.length, &input);

                match first_free_index {
                    Some(index) => {
                        if input[index].length == last.length {
                            input[index].content_type = last.content_type;
                            input[last_index].content_type = ContentType::Free;
                        } else {
                            input[index].length -= last.length;
                            input.insert(index, last);
                            input[last_index + 1].content_type = ContentType::Free;
                        }
                    }
                    None => {}
                }
            }
            ContentType::Free => {}
        }
    }

    println!("Part 2 solution: {}", helper::checksum(&input));
}

fn index_of_disk_content(disk_content: DiskContent, input: &Vec<DiskContent>) -> usize {
    input
        .iter()
        .enumerate()
        .find_map(|(index, inner_disk_content)| {
            if inner_disk_content.content_type == disk_content.content_type {
                Some(index)
            } else {
                None
            }
        })
        .unwrap()
}

fn first_free_index(
    max_index: usize,
    min_length: usize,
    input: &Vec<DiskContent>,
) -> Option<usize> {
    input.iter().enumerate().find_map(|(index, disk_content)| {
        if index < max_index
            && disk_content.length >= min_length
            && disk_content.content_type == ContentType::Free
        {
            Some(index)
        } else {
            None
        }
    })
}
