use regex::Regex;

/// Extracts words from a sentence, excluding punctuation characters
pub(crate) fn extract_words(sentence: & str) -> Vec<&str> {
    let mut words = vec![];
    let mut start_index = 0;
    let mut in_word = false;
    for (end_index, character) in sentence.char_indices() {
        if character.is_alphanumeric() {
            if !in_word {
                start_index = end_index;
                in_word = true;
            }
        } else if in_word {
            words.push(&sentence[start_index..end_index]);
            in_word = false;
        }
    }
    if in_word {
        words.push(&sentence[start_index..]);
    }
    words
}

/// Extract sentences from a text. Sentences are simply terminated by a full-stop.
pub(crate) fn extract_sentences(text: &str) -> Vec<&str> {
    let re = Regex::new(r"[\.\?!]+").unwrap();
    re.split(text)
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
        let words = extract_words(sentence);
        let mut words_iter = words.iter();
        assert_eq!(words_iter.next(), Some("I").as_ref());
        assert_eq!(words_iter.next(), Some("like").as_ref());
        assert_eq!(words_iter.next(), Some("trains").as_ref());
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
            &"Why does the lamb love Mary so"
        );
        assert_eq!(
            sentences_iter.next().unwrap(),
            &"The eager children cry; Why, Mary loves the lamb, you know, The teacher did reply"
        );
    }

    #[test]
    fn test_extract_sentences_with_ellipses() {
        let text = "I would like to come... But I don't want to! If only I could fly ?";
        let sentences = extract_sentences(text);
        let mut sentences_iter = sentences.iter();
        assert_eq!(sentences_iter.next().unwrap(), &"I would like to come");
        assert_eq!(sentences_iter.next().unwrap(), &"But I don't want to");
        assert_eq!(sentences_iter.next().unwrap(), &"If only I could fly");
    }
}
