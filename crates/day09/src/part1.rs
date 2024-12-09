use crate::{
    helper,
    model::{ContentType, DiskContent},
};

pub fn solve(input: &Vec<DiskContent>) {
    let mut input = input.clone();

    while !is_finished(&input) {
        let mut last = input.pop().unwrap();

        if last.content_type == ContentType::Free {
            continue;
        }

        let (first_free_index, first_free) = input
            .iter_mut()
            .enumerate()
            .find(|(_, disk_content)| disk_content.content_type == ContentType::Free)
            .unwrap();

        if first_free.length == last.length {
            first_free.content_type = last.content_type;
        } else if first_free.length > last.length {
            first_free.length -= last.length;
            input.insert(first_free_index, last);
        } else if first_free.length < last.length {
            first_free.content_type = last.content_type;
            last.length -= first_free.length;
            input.push(last);
        }
    }

    println!("Part 1 solution: {}", helper::checksum(&input));
}

fn is_finished(input: &Vec<DiskContent>) -> bool {
    let mut did_encounter_free_space = false;

    for index in 0..input.len() {
        match input[index].content_type {
            ContentType::File(_) => {
                if did_encounter_free_space {
                    return false;
                }
            }
            ContentType::Free => did_encounter_free_space = true,
        }
    }

    true
}
