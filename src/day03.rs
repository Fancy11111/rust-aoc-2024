use std::{fs, io::Lines};

pub fn day03_a(path: &str) -> std::io::Result<u64> {
    let file = fs::read_to_string(path)?;
    let lines = file.lines();
    let mut res: u64 = 0;
    for l in lines {
        let mut i: usize = 0;
        while i < l.len() {
            let cur = &l[i..];
            if !cur.starts_with("mul(") {
                i += 1;
                continue;
            }
            i += 4;
            // println!("{cur}");
            let cur = &cur[4..];
            let first_number = match parse_number(cur, b',') {
                Ok((val, offset)) => {
                    i += offset;
                    val
                }
                Err(offset) => {
                    // println!("could not parse first number: {offset}");
                    i += offset;
                    continue;
                }
            };
            let cur = &l[i..];
            // println!("success parsing first number, {first_number}");
            let second_number = match parse_number(cur, b')') {
                Ok((val, offset)) => {
                    i += offset;
                    val
                }
                Err(offset) => {
                    // println!("could not parse second number: {offset}, {cur}");
                    i += offset;
                    continue;
                }
            };
            // println!("success parsing mult({first_number},{second_number})");
            res += first_number * second_number;
        }
    }
    Ok(res)
}

type ParseNumberRes = Result<(u64, usize), usize>;

fn parse_number(cur: &str, end: u8) -> ParseNumberRes {
    let mut offset = 0;
    while offset < 3 && offset < cur.len() {
        let char: u8 = cur.as_bytes()[offset];
        if char.is_ascii_digit() {
            offset += 1;
        } else if char == end {
            let parsed: u64 = cur[..offset].parse().unwrap_or(0);
            return Ok((parsed, offset + 1));
        } else {
            return Err(offset);
        }
    }
    Err(offset)
}

pub fn day03_b(path: &str) -> std::io::Result<u64> {
    let file = fs::read_to_string(path)?;
    let lines = file.lines();
    let mut res: u64 = 0;
    for l in lines {
        let mut i: usize = 0;
        let mut enabled = true;
        while i < l.len() {
            let cur = &l[i..];
            if !enabled {
                if cur.starts_with("do()") {
                    enabled = true;
                    i += 4;
                } else {
                    i += 1;
                    continue;
                }
            } else if cur.starts_with("don't()") {
                enabled = false;
                i += "don't()".len();
                continue;
            }
            let cur = &l[i..];
            if !cur.starts_with("mul(") {
                i += 1;
                continue;
            }
            i += 4;
            // println!("{cur}");
            let cur = &cur[4..];
            let first_number = match parse_number(cur, b',') {
                Ok((val, offset)) => {
                    i += offset;
                    val
                }
                Err(offset) => {
                    // println!("could not parse first number: {offset}");
                    i += offset;
                    continue;
                }
            };
            let cur = &l[i..];
            // println!("success parsing first number, {first_number}");
            let second_number = match parse_number(cur, b')') {
                Ok((val, offset)) => {
                    i += offset;
                    val
                }
                Err(offset) => {
                    // println!("could not parse second number: {offset}, {cur}");
                    i += offset;
                    continue;
                }
            };
            // println!("success parsing mult({first_number},{second_number})");
            res += first_number * second_number;
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03_a() {
        match day03_a("data/day03.test.txt") {
            Ok(v) => assert_eq!(161, v, "Expected test data result to be 161, got `{v}`"),
            Err(e) => unreachable!("Got error `{e}`"),
        }
    }

    #[test]
    fn test_day03_b() {
        match day03_b("data/day03b.test.txt") {
            Ok(v) => assert_eq!(48, v, "Expected test data result to be 4, got `{v}`"),
            Err(e) => unreachable!("Got error `{e}`"),
        }
    }
}
