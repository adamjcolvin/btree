use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Node {
    pub max_keys: usize,
    pub keys: Vec<u32>,
    pub parent: Option<Rc<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            max_keys: 4,
            keys: vec![],
            parent: None,
            children: vec![],
        }
    }
}

impl Node {
    pub fn min_keys(&self) -> usize {
        self.max_keys / 2
    }

    pub fn insert(&mut self, value: u32) {
        if self.keys.len() == self.max_keys {
            self.split_node()
        } else {
            self.keys.push(value);
        }
    }

    fn split_node(&mut self) {
        self.parent = Some(Rc::new(RefCell::new(Node::default())));
        let split_node = Node {
            keys: self.keys.split_off(self.min_keys()),
            parent: self.parent.clone(),
            ..Default::default()
        };
        let child = Rc::new(RefCell::new(self));
        let split_child = Rc::new(RefCell::new(split_node));
        // if let Some(parent) = &mut self.parent {
        //     parent
        //         .borrow_mut()
        //         .children
        //         .append(&mut vec![child, split_child])
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_keys_per_node() {
        let node = Node {
            max_keys: 4,
            ..Node::default()
        };
        assert_eq!(node.max_keys, 4);
    }

    #[test]
    fn test_minimum_keys_per_node() {
        let node = Node {
            max_keys: 4,
            ..Node::default()
        };
        assert_eq!(node.min_keys(), 2);
    }

    #[test]
    fn test_insert() {
        let mut node = Node::default();
        assert_eq!(node.keys.len(), 0);
        node.insert(1);
        assert_eq!(node.keys.len(), 1);
        assert!(node.parent.is_none());
    }

    #[test]
    fn test_splitting_nodes() {
        //Create a node with the maximum keys (4)
        let mut node = Node {
            max_keys: 4,
            keys: vec![1, 2, 3, 4],
            parent: None,
            children: vec![],
        };
        //Assert that the node has no parent (root node)
        assert!(node.parent.is_none());
        //Call insert with a 5th key.
        node.insert(5);
        //Assert that the node now has a parent node with a single key.
        assert!(node.parent.is_some());
        //Assert that the parent has 2 child nodes.
        //Assert that the parent has the correct key.
        //Assert that the 5th key was added to the new child node.
    }

    #[test]
    fn test_number_of_child_nodes() {}
}
