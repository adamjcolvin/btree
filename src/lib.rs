mod node;
pub use node::Node;

pub struct BTree {
    child_nodes: Vec<Node>,
}

impl BTree {
    pub fn new(values: Vec<u32>) -> Self {
        let node = Node {
            max_keys: 4,
            keys: values,
            ..Node::default()
        };
        BTree {
            child_nodes: vec![node],
        }
    }

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

    #[test]
    fn test_root_node_minimum_keys() {}

    #[test]
    fn test_number_of_leaf_nodes() {
        let mut btree = BTree::new(vec![]);
        btree.insert(1);
        assert_eq!(btree.leaf_nodes().len(), 0);
    }

    #[test]
    fn test_leaf_nodes_at_same_level() {}

    #[test]
    fn test_balancing() {}
}
