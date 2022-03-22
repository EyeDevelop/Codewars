pub fn digital_root(mut n: i64) -> i64 {
    while (n as f64).log10() >= 1.0 {
        n = (0..=(n as f64).log10().ceil() as u32)
            .map(|exp| (n / 10_i64.pow(exp)) % 10)
            .reduce(|acc, item| acc + item)
            .unwrap();
    }

    n
}
