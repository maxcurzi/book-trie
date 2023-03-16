use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node<T> {
    element: Option<T>,
    count: usize,
    children: Option<HashMap<T, Node<T>>>,
}

impl<T: Eq + std::hash::Hash + Clone + Copy + std::fmt::Display> Node<T> {
    fn new() -> Self {
        Node {
            element: None,
            count: 0,
            children: None,
        }
    }

    fn add_sentence<I>(&mut self, sentence: I)
    where
        I: IntoIterator<Item = T>,
    {
        let mut words = sentence.into_iter();
        if let Some(word) = words.next() {
            let mut node = self
                .children
                .get_or_insert_with(HashMap::new)
                .entry(word.clone())
                .or_insert_with(|| Node {
                    element: Some(word),
                    count: 0,
                    children: None,
                });
            node.count += 1;
            for next_word in words {
                node = node
                    .children
                    .get_or_insert_with(HashMap::new)
                    .entry(next_word.clone())
                    .or_insert_with(|| Node {
                        element: Some(next_word),
                        count: 0,
                        children: None,
                    });
                node.count += 1;
            }
        }
    }

    fn get_child(&self, element: &T) -> Option<&Node<T>> {
        self.children
            .as_ref()
            .and_then(|children| children.get(element))
    }

    fn get_child_mut(&mut self, element: &T) -> Option<&mut Node<T>> {
        self.children
            .as_mut()
            .and_then(|children| children.get_mut(element))
    }

    fn print_tree_recursive(&self, prefix: &str, is_last: bool) {
        let current_prefix = if is_last {
            format!("{}└─ ", prefix)
        } else {
            format!("{}├─ ", prefix)
        };
        let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
        if let Some(ref element) = self.element {
            println!("{}{} (count: {})", current_prefix, element, self.count);
        }
        if let Some(ref children) = self.children {
            let sorted_children: Vec<(&T, &Node<T>)> = children.iter().collect();
            let len = sorted_children.len();
            for (index, child) in sorted_children.into_iter().enumerate() {
                let is_last_child = index == len - 1;
                child.1.print_tree_recursive(&child_prefix, is_last_child);
            }
        }
    }

    fn print_tree(&self) {
        self.print_tree_recursive("", true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut root = Node::new();
        root.add_sentence(vec![1, 2, 3, 4]);
        root.add_sentence(vec![1, 2, 3, 5]);
        root.add_sentence(vec![1, 2, 4, 5]);
        let mut root = Node::new();
        root.add_sentence("I like trains".split_ascii_whitespace());
        root.add_sentence("I like red potatoes".split_ascii_whitespace());
        assert_eq!(root.get_child(&"I").unwrap().element, Some("I"));
        assert_eq!(root.get_child(&"I").unwrap().count, 2);
        assert_eq!(
            root.get_child(&"I")
                .unwrap()
                .get_child(&"like")
                .unwrap()
                .element,
            Some("like")
        );
        assert_eq!(
            root.get_child(&"I")
                .unwrap()
                .get_child(&"like")
                .unwrap()
                .count,
            2
        );
        assert_eq!(
            root.get_child(&"I")
                .unwrap()
                .get_child(&"like")
                .unwrap()
                .get_child(&"trains")
                .unwrap()
                .element,
            Some("trains")
        );
        assert_eq!(
            root.get_child(&"I")
                .unwrap()
                .get_child(&"like")
                .unwrap()
                .get_child(&"trains")
                .unwrap()
                .count,
            1
        );
    }

    #[test]
    fn test_print() {
        let mut root = Node::new();
        root.add_sentence("I like trains".split_ascii_whitespace());
        root.add_sentence("I like red potatoes".split_ascii_whitespace());
        root.add_sentence("I like red wine".split_ascii_whitespace());
        root.add_sentence("I like green apples".split_ascii_whitespace());
        root.add_sentence("The quick brown fox jumps over the lazy dog".split_ascii_whitespace());
        root.add_sentence("The quick brown fox jumps over the lazy cat".split_ascii_whitespace());
        root.print_tree();
    }
}
