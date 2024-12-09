use reader::{read_file, File};

use crate::model::{ContentType, DiskContent};

pub fn parse() -> Vec<DiskContent> {
    let content = read_file(&File::Day09);

    const RADIX: u32 = 10;

    let output = content
        .char_indices()
        .map(|(index, ch)| {
            let length = ch.to_digit(RADIX).expect("Length must be a number!") as usize;

            if index % 2 == 0 {
                DiskContent {
                    content_type: ContentType::File(index / 2),
                    length,
                }
            } else {
                DiskContent {
                    content_type: ContentType::Free,
                    length,
                }
            }
        })
        .collect::<Vec<DiskContent>>();

    output
}
