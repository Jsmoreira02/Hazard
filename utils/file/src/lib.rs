use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn count_lines<P>(filename: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}