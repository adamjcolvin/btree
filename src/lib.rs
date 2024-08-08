mod node;
pub use node::Node;
use node::Nodeable;

pub struct BTree<T: Nodeable<T> = Node> {
    root_node: T,
}

impl<T: Default + Nodeable<T>> Default for BTree<T> {
    fn default() -> Self {
        BTree {
            root_node: T::default(),
        }
    }
}

impl<T: Clone + Nodeable<T>> BTree<T> {
    pub fn count(&self) -> usize {
        T::count(self.root_node.clone())
    }

    pub fn insert(&mut self, value: u32) {
        self.root_node = T::insert(value, self.root_node.clone())
            .expect("There was problem inserting the value");
    }

    pub fn bulk_insert(&mut self, values: Vec<u32>) {
        for value in values {
            self.insert(value);
        }
    }

    pub fn remove(&mut self, _value: u32) {
        // self.root_node = T::remove(value, self.root_node.clone())
        //     .expect("There was a problem removing the value");
    }

    pub fn leaf_nodes(&self) -> Vec<Node> {
        let leaf_nodes: Vec<Node> = vec![];
        leaf_nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct MockRootNode {
        insert_called: usize,
    }

    impl Default for MockRootNode {
        fn default() -> Self {
            MockRootNode { insert_called: 0 }
        }
    }

    impl Nodeable<MockRootNode> for MockRootNode {
        fn insert(_value: u32, into_node: MockRootNode) -> Option<MockRootNode> {
            let mut node = into_node.clone();
            node.insert_called += 1;
            Some(node)
        }

        fn count(_node: MockRootNode) -> usize {
            0
        }

        fn remove(_value: u32, from_node: MockRootNode) -> Option<MockRootNode> {
            Some(from_node.clone())
        }
    }

    #[test]
    fn test_root_node_minimum_keys() {}

    #[test]
    fn test_number_of_leaf_nodes() {
        let mut btree: BTree<MockRootNode> = BTree::default();
        btree.insert(1);
        assert_eq!(btree.leaf_nodes().len(), 0);
    }

    #[test]
    fn test_inserting() {
        let mut btree: BTree<MockRootNode> = BTree::default();
        btree.root_node = MockRootNode::default();
        btree.insert(1);
        assert_eq!(btree.root_node.insert_called, 1);
    }

    #[test]
    fn test_leaf_nodes_at_same_level() {}

    #[test]
    fn test_balancing() {}
}
