fn main() {
    println!("Hello, world!");
}

fn extract_words(sentence: &str) -> std::str::SplitWhitespace<'_> {
    sentence.trim_end_matches('.').split_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_words() {
        let sentence = "I like trains.";
        let mut words = extract_words(sentence);
        assert_eq!(words.next(), Some("I"));
        assert_eq!(words.next(), Some("like"));
        assert_eq!(words.next(), Some("trains"));
    }
}
