use std::fs;

pub fn day02_a(path: &str) -> std::io::Result<u32> {
    let file = fs::read_to_string(path)?;
    let file = file.lines().filter(|x| !x.is_empty());
    let mut res = 0;
    for l in file.map(|l| {
        l.split(" ")
            .map(|v| v.parse::<u32>())
            .filter_map(Result::ok)
    }) {
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
                .map(|v| v.parse::<i32>())
                .filter_map(Result::ok)
        })
        .enumerate()
    {
        let diffs = l.clone().zip(l.clone().skip(1)).map(|(p, c)| p - c);
        let all_negative = diffs.clone().all(|x| x < 0);
        let all_positive = diffs.clone().all(|x| x > 0);
        let all_in_range = diffs.clone().all(|x| (1..=3).contains(&x.abs()));
        if all_in_range && (all_positive || all_negative) {
            res += 1;
            continue;
        }
        let l = l.collect::<Vec<i32>>();
        let mut j = 0;
        let mut safe = false;
        while j < l.len() && !safe {
            let without_j = l
                .clone()
                .into_iter()
                .enumerate()
                .filter_map(|(idx, v)| if idx != j { Some(v) } else { None });
            let diffs = without_j
                .clone()
                .zip(without_j.clone().skip(1))
                .map(|(p, c)| p - c);
            let all_negative = diffs.clone().all(|x| x < 0);
            let all_positive = diffs.clone().all(|x| x > 0);
            let all_in_range = diffs.clone().all(|x| (1..=3).contains(&x.abs()));
            if all_in_range && (all_positive || all_negative) {
                res += 1;
                safe = true;
            }
            j += 1;
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
            Ok(v) => assert_eq!(2, v, "Expected test data result to be 2, got `{v}`"),
            Err(e) => unreachable!("Got error `{e}`"),
        }
    }

    #[test]
    fn test_day02_b() {
        match day02_b("data/day02.test.txt") {
            Ok(v) => assert_eq!(4, v, "Expected test data result to be 4, got `{v}`"),
            Err(e) => unreachable!("Got error `{e}`"),
        }
    }
}
