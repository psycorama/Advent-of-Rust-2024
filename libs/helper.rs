use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("unable to parse line"))
        .collect()
}


pub fn get_2d_vector_from_file(filename: impl AsRef<Path>) ->Vec<Vec<char>> {

    let buffer = lines_from_file(filename);

    let mut input: Vec<Vec<char>> = vec![];
    for line in buffer {
        input.push( line.chars().collect() );
    }
    input
}
