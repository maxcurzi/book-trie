/// Extracts words from a sentence which may be terminated by a full-stop
pub(crate) fn extract_words(sentence: &str) -> std::str::SplitWhitespace<'_> {
    sentence.trim_end_matches('.').split_whitespace()
}

/// Extract sentences from a text. Sentences are simply terminated by a full-stop.
pub(crate) fn extract_sentences(text: &str) -> Vec<&str> {
    text.split('.')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect()
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

    #[test]
    fn test_extract_sentences() {
        let text =
"Mary had a little lamb, Its fleece was white as snow; And everywhere that Mary went The lamb was sure to go.
It followed her to school one day, Which was against the rule; It made the children laugh and play To see a lamb at school.
And so the teacher turned it out, But still it lingered near, And waited patiently about Till Mary did appear.
Why does the lamb love Mary so? The eager children cry; Why, Mary loves the lamb, you know, The teacher did reply.";
        let sentences = extract_sentences(text);
        let mut sentences_iter = sentences.iter();
        assert_eq!(
            sentences_iter.next().unwrap(),
            &"Mary had a little lamb, Its fleece was white as snow; And everywhere that Mary went The lamb was sure to go"
        );
        assert_eq!(
            sentences_iter.next().unwrap(),
            &"It followed her to school one day, Which was against the rule; It made the children laugh and play To see a lamb at school"
        );
        assert_eq!(
            sentences_iter.next().unwrap(),
            &"And so the teacher turned it out, But still it lingered near, And waited patiently about Till Mary did appear"
        );
        assert_eq!(
            sentences_iter.next().unwrap(),
            &"Why does the lamb love Mary so? The eager children cry; Why, Mary loves the lamb, you know, The teacher did reply"
        );
    }

    #[test]
    fn test_extract_sentences_with_ellipses() {
        let text = "I would like to come... But I don't want to!";
        let sentences = extract_sentences(text);
        let mut sentences_iter = sentences.iter();
        assert_eq!(sentences_iter.next().unwrap(), &"I would like to come");
        assert_eq!(sentences_iter.next().unwrap(), &"But I don't want to!");
    }
}
