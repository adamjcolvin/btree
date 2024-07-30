#[derive(Clone)]
pub struct Node {
    pub max_keys: usize,
    pub keys: Vec<u32>,
    pub children: Vec<Box<Node>>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            max_keys: 4,
            keys: vec![],
            children: vec![],
        }
    }
}

impl Node {
    fn new() -> Self {
        Node {
            keys: vec![],
            children: vec![],
            ..Default::default()
        }
    }
    pub fn min_keys(&self) -> usize {
        self.max_keys / 2
    }

    pub fn insert(&mut self, value: u32) {
        self.keys.push(value);
        if self.keys.len() >= self.max_keys {
            self.split();
        }
    }

    fn split(&mut self) {
        let middle = self.keys.len() / 2;
        let middle_value = self.keys[middle].clone();

        let mut left_node = Node::new();
        left_node.keys.extend(self.keys.drain(..middle));

        let mut right_node = Node::new();
        right_node.keys.extend(self.keys.drain(1..));

        self.keys.push(middle_value);
        self.children.push(Box::new(left_node));
        self.children.push(Box::new(right_node));
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
    }

    #[test]
    fn test_splitting_nodes() {
        //Create a node with the maximum keys (4)
        let mut node = Node {
            max_keys: 4,
            keys: vec![1, 2, 3, 4],
            children: vec![],
        };
        //Call insert with a 5th key.
        node.insert(5);
        //Assert that the node now has two children with the correct keys.
        assert_eq!(node.children.len(), 2);
        //Assert that the parent has the correct key.
        //Assert that the 5th key was added to the new child node.
    }

    #[test]
    fn test_number_of_child_nodes() {}
}
