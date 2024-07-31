#[derive(Clone, Debug)]
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
        if self.children.len() == 0 {
            self.keys.push(value);
            self.keys.sort();
            if self.keys.len() > self.max_keys {
                self.split();
            }
        } else {
            for child in self.children.iter_mut().rev() {
                let lowest_key = child.keys.first().unwrap();
                if value >= *lowest_key {
                    child.insert(value);
                    break;
                }
            }
        }
    }

    fn split(&mut self) {
        let key_middle_index = self.keys.len() / 2;
        let middle_key = self.keys[key_middle_index].clone();
        let child_middle_index = self.children.len() / 2;

        let mut left_node = Node::new();
        left_node.keys.extend(self.keys.drain(..key_middle_index));
        left_node
            .children
            .extend(self.children.drain(..child_middle_index));

        let mut right_node = Node::new();
        right_node.keys.extend(self.keys.drain(1..));
        right_node.children.extend(self.children.drain(..));

        self.keys.push(middle_key);
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
    fn test_insert_into_child_node() {
        let child_1 = Box::new(Node {
            keys: vec![4, 5, 6],
            ..Default::default()
        });
        let child_2 = Box::new(Node {
            keys: vec![7, 8, 9],
            ..Default::default()
        });
        let mut node = Node {
            max_keys: 4,
            keys: vec![1, 2, 3],
            children: vec![child_1, child_2],
        };
        node.insert(10);
        assert_eq!(node.keys, vec![1, 2, 3]);
        assert_eq!(node.children.len(), 2);
        let child_1 = node.children.first().unwrap();
        assert_eq!(*child_1.keys, vec![4, 5, 6]);
        let child_2 = node.children.last().unwrap();
        assert_eq!(*child_2.keys, vec![7, 8, 9, 10]);
    }

    #[test]
    fn test_splitting_leaf_nodes() {
        let mut node = Node {
            max_keys: 4,
            keys: vec![1, 2, 3, 4],
            children: vec![],
        };
        node.insert(5);
        assert_eq!(node.children.len(), 2);
        let expected_parent_key = node.keys.last().unwrap();
        assert_eq!(*expected_parent_key, 3);
        let expected_child_key = node.children.last().unwrap().keys.last().unwrap();
        assert_eq!(*expected_child_key, 5);
    }

    #[test]
    fn test_number_of_child_nodes() {}
}
