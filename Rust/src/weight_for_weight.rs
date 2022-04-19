pub fn weight_for_weight(s: &str) -> String {
    let mut arr = s.split_whitespace()
        .collect::<Vec<&str>>();
    
    arr.sort_by_cached_key(|item| 
        (item.chars()
            .map(|c| c.to_digit(10)
                .unwrap())
            .sum::<u32>(), item.to_owned()));

    arr.join(" ")
}