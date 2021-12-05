use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn read_lines_as_vec<R, P>(filename: P) -> Result<Vec<R>>
where
    R: FromStr,
    P: AsRef<Path>,
{
    let lines = read_lines(filename)?;

    Ok(lines
        .filter_map(|l| {
            if let Ok(line) = l {
                return line.parse().ok();
            }

            None
        })
        .collect())
}
