pub trait Nodeable {
    fn insert(value: u32, into_node: Node) -> Node;
}

#[derive(Clone, Debug)]
pub struct Node {
    pub max_keys: usize,
    pub keys: Vec<u32>,
    pub children: Vec<Node>,
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

impl Nodeable for Node {
    fn insert(value: u32, into_node: Node) -> Node {
        let node = into_node.clone();
        let mut new_node = Node {
            keys: node.keys.clone(),
            children: vec![],
            ..Default::default()
        };
        if node.children.len() == 0 {
            new_node.keys.push(value);
            new_node.keys.sort();
            if new_node.keys.len() > node.max_keys {
                new_node = Node::split(new_node);
            }
        } else {
            new_node
                .children
                .extend(Node::insert_into_children(value, node.children.clone()));
        }
        new_node
    }
}

impl Node {
    pub fn new(keys: Vec<u32>) -> Self {
        Node {
            keys,
            ..Default::default()
        }
    }

    pub fn min_keys(&self) -> usize {
        self.max_keys / 2
    }

    fn insert_into_children(value: u32, children: Vec<Node>) -> Vec<Node> {
        let mut new_children: Vec<Node> = vec![];
        for (index, child) in children.iter().enumerate().rev() {
            if let Some(lowest_key) = child.keys.first() {
                if value >= *lowest_key {
                    let new_child = Node::insert(value, child.clone());
                    new_children.insert(0, new_child);
                    let remaining_children = &children[..index].to_vec();
                    new_children = remaining_children
                        .iter()
                        .cloned()
                        .chain(new_children)
                        .collect();
                    break;
                } else {
                    new_children.insert(0, child.clone());
                }
            }
        }
        new_children
    }

    fn split(node: Node) -> Node {
        let key_middle_index = node.keys.len() / 2;
        let middle_key = node.keys[key_middle_index].clone();
        let mut keys = node.keys.clone();
        let split_node = Node::new(keys.drain(..key_middle_index).collect());
        let right_node = Node::new(keys.drain(..).collect());
        Node {
            keys: vec![middle_key],
            children: vec![split_node.clone(), right_node.clone()],
            ..Node::default()
        }
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
        let node = Node::default();
        assert_eq!(node.keys.len(), 0);
        let updated_node = Node::insert(5, node);
        assert_eq!(updated_node.keys.len(), 1);
        assert_eq!(*updated_node.keys.first().unwrap(), 5);
    }

    #[test]
    fn test_insert_into_child_node() {
        let child_1 = Node::new(vec![4, 5, 6]);
        let child_2 = Node::new(vec![7, 8, 9]);
        let node = Node {
            max_keys: 4,
            keys: vec![1, 2, 3],
            children: vec![child_1, child_2],
            ..Default::default()
        };
        let inserted_node = Node::insert(10, node);
        assert_eq!(inserted_node.keys, vec![1, 2, 3]);
        assert_eq!(inserted_node.children.len(), 2);
        if let Some(child_1) = inserted_node.children.first() {
            assert_eq!(child_1.keys, vec![4, 5, 6]);
        }
        if let Some(child_2) = inserted_node.children.last() {
            assert_eq!(child_2.keys, vec![7, 8, 9, 10]);
        }
    }

    #[test]
    fn test_splitting_leaf_nodes() {
        let node = Node {
            max_keys: 4,
            keys: vec![1, 2, 3, 4],
            children: vec![],
        };
        let new_parent_node = Node::insert(5, node);
        assert_eq!(new_parent_node.children.len(), 2);
        if let Some(parent_key) = new_parent_node.keys.first() {
            assert_eq!(*parent_key, 3);
        } else {
            panic!("Expected to find a key of value 3");
        }
        if let Some(last_child) = new_parent_node.children.last() {
            if let Some(last_key) = last_child.keys.last() {
                assert_eq!(*last_key, 5);
            } else {
                panic!("Expected to find a key value 5");
            }
        } else {
            panic!("Expected to find a child in the new parent node.")
        }
    }

    #[test]
    fn test_number_of_child_nodes() {}
}
