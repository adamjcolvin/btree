mod node;
pub use node::Node;
use node::Nodeable;

pub struct BTree<T: Nodeable> {
    root_node: T,
    child_nodes: Vec<Node>,
}

impl<T: Default + Nodeable> Default for BTree<T> {
    fn default() -> Self {
        BTree {
            root_node: T::default(),
            child_nodes: vec![],
        }
    }
}

impl<T: Nodeable> BTree<T> {
    pub fn count(&self) -> usize {
        self.child_nodes
            .clone()
            .into_iter()
            .map(|node| node.keys.len())
            .sum()
    }

    pub fn insert(&mut self, value: u32) {
        if self.child_nodes.len() == 0 {
            let node = Node {
                max_keys: 4,
                keys: vec![value],
                ..Node::default()
            };
            self.child_nodes.push(node);
        } else {
            self.child_nodes[0].keys.push(value);
        }
    }

    pub fn bulk_insert(&mut self, values: Vec<u32>) {
        if self.child_nodes.len() == 0 {
            let node = Node {
                max_keys: 4,
                keys: values,
                ..Node::default()
            };
            self.child_nodes.push(node);
        } else {
            self.child_nodes[0].keys.extend(values);
        }
    }

    pub fn remove(&mut self, value: u32) {
        for node in &mut self.child_nodes {
            if let Some(index) = node.keys.iter().position(|&x| x == value) {
                node.keys.remove(index);
            }
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

    struct MockRootNode {}

    impl Default for MockRootNode {
        fn default() -> Self {
            MockRootNode {}
        }
    }

    impl Nodeable for MockRootNode {
        fn insert(value: u32, into_node: Node) -> Node {
            into_node
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

    fn test_inserting() {
        let mut btree: BTree<MockRootNode> = BTree::default();
        btree.root_node = MockRootNode {};
        //Inject the root node here using a mock struct.
        //Test the return value of insert.
        //Test that the root node was used and the correct method called.
    }

    #[test]
    fn test_leaf_nodes_at_same_level() {}

    #[test]
    fn test_balancing() {}
}
