mod text_parser;
mod trie;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use text_parser::extract_sentences;
use text_parser::extract_words;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to process
    #[arg(short, long)]
    in_file: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut reader = Box::new(BufReader::new(File::open(args.in_file)?));
    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    let mut root = trie::Node::new();
    for sentence in extract_sentences(&text) {
        root.add_sentence(extract_words(sentence));
    }
    root.print_tree(&mut std::io::stdout());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::text_parser::{extract_sentences, extract_words};

    use super::*;

    #[test]
    fn test_create_trie() {
        let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        let mut root = trie::Node::new();
        let sentences = extract_sentences(text);
        for sentence in sentences {
            let words = extract_words(sentence);
            root.add_sentence(words);
        }
        root.print_tree(&mut std::io::stdout());
    }

    #[test]
    fn test_read_text_from_file() {
        let tmp_file = "tmp/test.txt";
        let mut file = File::create(tmp_file).unwrap();
        let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        file.write_all(text.as_bytes()).unwrap();

        let file = File::open(tmp_file).unwrap();
        let mut reader = BufReader::new(file);
        let mut text = String::new();
        reader.read_to_string(&mut text).unwrap();
        fs::remove_file(tmp_file).unwrap();
        let mut root = trie::Node::new();
        for sentence in extract_sentences(&text) {
            root.add_sentence(extract_words(sentence));
        }
        root.print_tree(&mut std::io::stdout());
    }
}
