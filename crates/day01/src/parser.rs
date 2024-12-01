use reader::{read_file, File};

pub fn parse() -> (Vec<usize>, Vec<usize>) {
    let content = read_file(File::Day01);

    let output = content
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();

            let first = split.next().unwrap().parse::<usize>().unwrap();
            let second = split.next().unwrap().parse::<usize>().unwrap();

            (first, second)
        })
        .fold((Vec::new(), Vec::new()), |mut acc, (first, second)| {
            acc.0.push(first);
            acc.1.push(second);

            acc
        });

    return output;
}
