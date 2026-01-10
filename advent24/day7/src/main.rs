use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_input(input_reader: BufReader<File>) {
    let mut lines = input_reader.lines();

    while let Some(Ok(line)) = lines.next() {
        println!("{}", line)
    }
}

fn main() {
    let file = File::open("input_test").unwrap();
    let input_reader = BufReader::new(file);

    parse_input(input_reader);
}
