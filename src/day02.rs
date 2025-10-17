use std::fs;

pub fn day02_a(path: &str) -> std::io::Result<u32> {
    let file = fs::read_to_string(path)?;
    let file = file.lines().filter(|x| !x.is_empty());
    let mut res = 0;
    for (i, l) in file
        .map(|l| {
            l.split(" ")
                .map(|v| v.parse::<u32>())
                .filter_map(Result::ok)
        })
        .enumerate()
    {
        let offset_line = l.clone();
        let offset_line = offset_line.skip(1);
        let zipped = l.clone().zip(offset_line);
        let asc = zipped.clone().take(1).all(|(c, n)| c < n);
        if zipped.into_iter().all(|(c, n)| {
            let diff = c.abs_diff(n);
            (1..=3).contains(&diff) && ((asc && c < n) || (!asc && c > n))
        }) {
            res += 1;
        }
    }
    Ok(res)
}

pub fn day02_b(path: &str) -> std::io::Result<u32> {
    let file = fs::read_to_string(path)?;
    let file = file.lines().filter(|x| !x.is_empty());
    let mut res = 0;
    for (i, l) in file
        .map(|l| {
            l.split(" ")
                .map(|v| v.parse::<u32>())
                .filter_map(Result::ok)
        })
        .enumerate()
    {
        let values = l.collect::<Vec<u32>>();
        let mut l = 0;
        let mut r = 1;
        let mut asc: Option<bool> = None;
        while l + 2 >= r && values.len() > r {
            let left = match values.get(l) {
                Some(v) => v,
                None => break,
            };
            let right = match values.get(r) {
                Some(v) => v,
                None => break,
            };

            let asc_val = if asc.is_none() {
                if left != right {
                    let res = left > right;
                    asc = Some(res);
                    res
                } else {
                    println!("{i}: {left} == {right}");
                    r += 1;
                    continue;
                }
            } else {
                asc.unwrap_or(false)
            };

            let diff = left.abs_diff(*right);

            if asc_val && l <= r {
                r += 1;
                println!("{i}: {left} >= {right}");
                continue;
            } else if !asc_val && l >= r {
                println!("{i}: {left} <= {right}");
                r += 1;
                continue;
            } else if !(1..=3).contains(&diff) {
                println!("{i}: {left} - {right}");
                r += 1;
                continue;
            }

            l += 1;
            r += 1;
        }
        if l + 2 >= r && values.len() == r {
            res += 1;
            println!("safe {i}: {l}, {r}, {}", values.len());
        } else {
            println!("unsafe {i}: {l}, {r}, {}", values.len());
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02_a() {
        match day02_a("data/day02.test.txt") {
            Ok(v) => assert_eq!(v, 2, "Expected test data result to be 2, got `{v}`"),
            Err(e) => unreachable!("Got error `{e}`"),
        }
    }

    #[test]
    fn test_day02_b() {
        match day02_b("data/day02.test.txt") {
            Ok(v) => assert_eq!(v, 4, "Expected test data result to be 2, got `{v}`"),
            Err(e) => unreachable!("Got error `{e}`"),
        }
    }
}
