pub fn highest_lowest(numbers: &str) -> String {
    let n = numbers
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .fold((None, None), |acc, item| {
            match (acc, item) {
                ((Some(n), r), i) if i < n => (Some(i), r),
                ((r, Some(n)), i) if i > n => (r, Some(i)),
                ((None, None), i) => (Some(i), Some(i)),
                _ => acc,
            }
        });

    format!("{} {}", n.1.unwrap(), n.0.unwrap())
}
