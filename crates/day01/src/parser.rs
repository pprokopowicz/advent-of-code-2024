use reader::{read_file, File};

pub fn parse() -> (Vec<usize>, Vec<usize>) {
    let content = read_file(File::Day01);

    let mut first_column = vec![];
    let mut second_column = vec![];

    content.lines().for_each(|line| {
        let mut split = line.split_whitespace();

        let first_num = split
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("First part is not a number!");

        let second_num = split
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Second part is not a number!");

        first_column.push(first_num);
        second_column.push(second_num);
    });

    return (first_column, second_column);
}
