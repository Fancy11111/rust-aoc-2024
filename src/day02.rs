use std::fs;

pub fn day02_a(path: &str) -> std::io::Result<u32> {
    let file = fs::read_to_string(path)?;
    let mut res = 0;
    for (i, l) in file
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|v| v.parse::<u32>())
                .filter_map(Result::ok)
        })
        .enumerate()
    {
        let offset_line = l.clone();
        let offset_line = offset_line.skip(1);
        let zipped = l.zip(offset_line);
        let asc = zipped.clone().take(1).all(|(c, n)| c < n);
        if zipped.into_iter().all(|(c, n)| {
            let diff = c.abs_diff(n);
            (1..=3).contains(&diff) && ((asc && c < n) || (!asc && c > n))
        }) {
            res += 1;
            println!("safe {i}");
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
}
