use std::{fmt, fs::read_to_string};

#[derive(Debug, Clone)]
struct FileFormatError;

impl fmt::Display for FileFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected file format")
    }
}

pub fn day01_a(path: &str) -> std::io::Result<u32> {
    let file_content = read_to_string(path)?;
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for line in file_content.lines() {
        let split: Vec<u32> = line
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>())
            .filter_map(Result::ok)
            .collect();
        if split.is_empty() {
            continue;
        }
        if split.len() != 2 {
            eprintln!(
                "File format unexpected, expected two numbers per line, found {}",
                split.len()
            );
            return Ok(0); // TODO: better error
            // return Err(FileFormatError);
        }

        if let Some(first) = split.first() {
            left.push(*first);
        }

        if let Some(second) = split.get(1) {
            right.push(*second);
        }
    }
    left.sort();
    right.sort();

    let mut distance = 0;
    for (l, r) in left.iter().zip(right) {
        distance += l.abs_diff(r)
    }

    println!("distance: {distance}");

    Ok(distance)
}

// TODO: day1_b
pub fn day01_b() -> std::io::Result<u32> {
    let file_content = read_to_string("data/day01.test.txt")?;
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for line in file_content.lines() {
        let split: Vec<u32> = line
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>())
            .filter_map(Result::ok)
            .collect();
        if split.is_empty() {
            continue;
        }
        if split.len() != 2 {
            eprintln!(
                "File format unexpected, expected two numbers per line, found {}",
                split.len()
            );
            return Ok(1); // TODO: better error
        }

        if let Some(first) = split.first() {
            left.push(*first);
        }

        if let Some(second) = split.get(1) {
            right.push(*second);
        }
    }
    left.sort();
    right.sort();

    let mut distance = 0;
    for (l, r) in left.iter().zip(right) {
        distance += l.abs_diff(r)
    }

    println!("distance: {distance}");

    Ok(distance)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day01a() {
        match day01_a("data/day01.test.txt") {
            Ok(v) => assert_eq!(v, 12, "Expected test data result to be 11, got `{v}`"),
            Err(e) => assert!(false, "Got error `{e}`"),
        };
    }
}
