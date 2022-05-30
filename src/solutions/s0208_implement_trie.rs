use std::collections::HashMap;

#[derive(Default)]
struct Node {
    end_word: bool,
    children: HashMap<u8, Box<Node>>,
}

#[derive(Default)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;

        for c in word.as_bytes() {
            node = node.children.entry(*c).or_default();
        }
        node.end_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        match self.get_tail(&word) {
            Some(node) => node.end_word,
            _ => false,
        }
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.get_tail(&prefix).is_some()
    }

    fn get_tail(&self, prefix: &str) -> Option<&Node> {
        let mut node = &self.root;

        for c in prefix.as_bytes() {
            match node.children.get(c) {
                Some(n) => node = n,
                _ => return None,
            }
        }

        match std::ptr::eq(node, &self.root) {
            true => None,
            false => Some(node),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }

    #[test]
    fn test2() {
        let trie = Trie::new();
        assert!(!trie.starts_with("app".to_string()));
    }

    #[test]
    fn test3() {
        let mut trie = Trie::new();
        trie.insert("hello".to_string());
        assert!(!trie.search("hell".to_string()));
        assert!(!trie.search("helloa".to_string()));
        assert!(trie.search("hello".to_string()));
        assert!(trie.starts_with("hell".to_string()));
        assert!(!trie.starts_with("helloa".to_string()));
        assert!(trie.starts_with("hello".to_string()));
    }
}
