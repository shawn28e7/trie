pub mod trie
{
    struct Node
    {
        children: Vec<Option<usize>>,
        id: Option<i32>,
        cover: i32,
    }

    pub struct Trie
    {
        nodes: Vec<Node>,
    }

    impl Node
    {
        pub fn new() -> Node
        {
            Node {
                children: vec![None; 52],
                id: None,
                cover: 0,
            }
        }
    }

    impl Trie
    {
        pub fn new() -> Trie
        {
            Trie {
                nodes: vec![Node::new()],
            }
        }

        pub fn insert(&mut self, word: &str, id: i32)
        {
            let mut current_node_index: usize = 0;
            for &byte in word.as_bytes()
            {
                let index: usize = self.char_to_index(byte);
                if self.nodes[current_node_index].children[index].is_none()
                {
                    self.nodes[current_node_index].children[index] = Some(self.nodes.len());
                    self.nodes.push(Node::new());
                }
                current_node_index = self.nodes[current_node_index].children[index].unwrap();
                self.nodes[current_node_index].cover += 1;
            }

            self.nodes[current_node_index].id = Some(id);
        }

        pub fn search(&self, word: &str) -> Option<i32>
        {
            let mut current_node_index: usize = 0;
            for &byte in word.as_bytes()
            {
                let index = self.char_to_index(byte);
                if let Some(next_node_index) = self.nodes[current_node_index].children[index]
                {
                    current_node_index = next_node_index;
                }
                else
                {
                    return None;
                }
            }
            self.nodes[current_node_index].id
        }
        pub fn delete(&mut self, word: &str) -> bool
        {
            return self.delete_recursively(word.as_bytes(), 0, 0);
        }
        fn delete_recursively(
            &mut self,
            word: &[u8],
            current_node_index: usize,
            depth: usize,
        ) -> bool
        {
            if depth == word.len()
            {
                self.nodes[current_node_index].id = None;
                self.nodes[current_node_index].cover -= 1;
                return true;
            }
            let index = self.char_to_index(word[depth]);
            if let Some(next_node_index) = self.nodes[current_node_index].children[index]
            {
                if self.delete_recursively(word, next_node_index, depth + 1)
                {
                    if self.nodes[next_node_index].cover == 0
                    {
                        self.nodes[current_node_index].children[index] = None;
                    }
                    self.nodes[current_node_index].cover -= 1;
                    return true;
                }
                else
                {
                    return false;
                }
            }
            return false;
        }

        fn char_to_index(&self, c: u8) -> usize
        {
            match c
            {
                b'a'..=b'z' => (c - b'a') as usize,
                b'A'..=b'Z' => (c - b'A' + 26) as usize,
                _ => panic!("Unsupported character: {}", c),
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::trie::Trie;

    #[test]
    fn test_insert_and_search()
    {
        let mut trie = Trie::new();
        assert_eq!(trie.search("hello"), None);

        trie.insert("hello", 1);
        assert_eq!(trie.search("hello"), Some(1));
        assert_eq!(trie.search("hell"), None);
        assert_eq!(trie.search("helloo"), None);

        trie.insert("hell", 2);
        assert_eq!(trie.search("hell"), Some(2));
        assert_eq!(trie.search("helloo"), None);

        trie.insert("HELLO", 3);
        assert_eq!(trie.search("HELLO"), Some(3));
        assert_eq!(trie.search("hello"), Some(1));
        assert_eq!(trie.search("hell"), Some(2));

        trie.insert("Hi", 4);
        assert_eq!(trie.search("Hi"), Some(4));
        assert_eq!(trie.search("H"), None);
        assert_eq!(trie.search("h"), None);
        assert_eq!(trie.search("hi"), None);
    }
    #[test]
    fn test_delete()
    {
        let mut trie = Trie::new();

        trie.insert("hello", 1);
        trie.insert("hell", 2);
        trie.insert("hi", 3);

        assert_eq!(trie.search("hello"), Some(1));
        assert_eq!(trie.search("hell"), Some(2));
        assert_eq!(trie.search("hi"), Some(3));

        assert!(trie.delete("hello"));
        assert_eq!(trie.search("hello"), None);
        assert_eq!(trie.search("hell"), Some(2));
        assert_eq!(trie.search("hi"), Some(3));

        assert!(trie.delete("hell"));
        assert_eq!(trie.search("hell"), None);
        assert_eq!(trie.search("hi"), Some(3));

        assert!(!trie.delete("hello"));
        assert_eq!(trie.search("hi"), Some(3));

        assert!(trie.delete("hi"));
        assert_eq!(trie.search("hi"), None);
    }
    #[test]
    #[should_panic(expected = "Unsupported character: 33")]
    fn test_unsupported_character()
    {
        let mut trie = Trie::new();
        trie.insert("hello!", 1);
    }
}

fn main()
{
    // Example usage of the Trie
    let mut trie = trie::Trie::new();
    trie.insert("example", 42);
    println!("ID of 'example': {:?}", trie.search("example"));
    trie.delete("example");
    println!(
        "ID of 'example' after deletion: {:?}",
        trie.search("example")
    );
}
