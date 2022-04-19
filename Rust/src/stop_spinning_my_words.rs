pub fn stop_spinning_my_words(words: &str) -> String {
    words.split_whitespace()
        .map(|w| if w.len() > 4 { w.chars().rev().collect::<String>() } else { w.to_owned() })
        .collect::<Vec<_>>()
        .join(" ")
}