mod node;
pub use node::Node;

pub struct BTree {
    child_nodes: Vec<Node>
}

impl BTree {
    pub fn new() -> Self {
        BTree {
            child_nodes: vec![]
        }
    }

    pub fn count(&self) -> u32 {
        0
    }

    pub fn insert(&mut self, value: u32) {
        if self.child_nodes.len() == 0 {
            let node = Node { keys: vec![value] };
            self.child_nodes.push(node);
        } else {
            self.child_nodes[0].keys.push(value);
        }
    }

    pub fn leaf_nodes(&self) -> Vec<Node> {
        let leaf_nodes: Vec<Node> = vec![];
        leaf_nodes
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_empty_btree() {
        assert_eq!(BTree::new().count(), 0);
    }

    #[test]
    fn test_number_of_leaf_nodes() {
        let mut btree = BTree::new();
        btree.insert(1);
        assert_eq!(btree.leaf_nodes().len(), 0);
    }

    #[test]
    fn test_balancing() {
    }

    #[test]
    fn test_inserting_element() {
    }

    #[test]
    fn test_inserting_multiple_elements() {
    }

    #[test]
    fn test_removing_element() {
    }

    #[test]
    fn test_removing_multiple_elements() {
    }
}
