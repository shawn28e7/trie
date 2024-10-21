pub mod trie
{
    use std::cell::RefCell;
    use std::rc::Rc;

    struct Node
    {
        children: Vec<Option<Rc<RefCell<Node>>>>,
        id: Option<i32>,
    }

    pub struct Trie
    {
        root: Rc<RefCell<Node>>,
    }

    impl Node
    {
        pub fn new() -> Rc<RefCell<Node>>
        {
            Rc::new(RefCell::new(Node {
                children: vec![None; 52],
                id: None,
            }))
        }

        fn insert(&mut self, key: &str, id: i32)
        {
            if key.is_empty()
            {
                self.id = Some(id);
                return;
            }
            let key = key.as_bytes();
            let index = self.char_to_index(&key[0]);
            if self.children[index].is_none()
            {
                self.children[index] = Some(Node::new());
            }
            self.children[index]
                .as_ref()
                .unwrap()
                .borrow_mut()
                .insert(std::str::from_utf8(&key[1..]).unwrap(), id);
        }
        fn search(&self, key: &str) -> Option<i32>
        {
            if key.is_empty()
            {
                return self.id;
            }
            let key = key.as_bytes();
            let index = self.char_to_index(&key[0]);
            if self.children[index].is_none()
            {
                return None;
            }
            self.children[index]
                .as_ref()
                .unwrap()
                .borrow()
                .search(std::str::from_utf8(&key[1..]).unwrap())
        }
        fn delete(&mut self, key: &str) -> (bool, bool)
        {
            let delete_success: bool = if key.is_empty()
            {
                if self.id.is_none()
                {
                    false
                }
                else
                {
                    self.id = None;
                    true
                }
            }
            else
            {
                let key = key.as_bytes();
                let index = self.char_to_index(&key[0]);
                if self.children[index].is_none()
                {
                    false
                }
                else
                {
                    let child = self.children[index].as_ref().unwrap();
                    let res = child
                        .borrow_mut()
                        .delete(std::str::from_utf8(&key[1..]).unwrap());
                    if res.1
                    {
                        self.children[index] = None;
                    }
                    res.0
                }
            };
            let node_useless = self.children.iter().all(|x| x.is_none());
            (delete_success, node_useless)
        }
        fn char_to_index(&self, c: &u8) -> usize
        {
            match c
            {
                b'a'..=b'z' => (c - b'a') as usize,
                b'A'..=b'Z' => (c - b'A' + 26) as usize,
                _ => panic!("Unsupported character: {}", c),
            }
        }
    }

    impl Trie
    {
        pub fn new() -> Trie
        {
            Trie {
                root: Node::new(), // Initialize the root node
            }
        }

        pub fn insert(&mut self, key: &str, id: i32)
        {
            self.root.borrow_mut().insert(key, id);
        }
        pub fn search(&self, key: &str) -> Option<i32>
        {
            self.root.borrow().search(key)
        }
        pub fn delete(&mut self, key: &str) -> bool
        {
            self.root.borrow_mut().delete(key).0
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
