use std::cmp::Ordering;

pub trait Nodeable<T: Nodeable<T>> {
    fn insert(value: u32, into_node: T) -> Option<T>;
    fn remove(value: u32, from_node: T) -> Option<T>;
    fn count(node: T) -> usize;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    pub max_keys: usize,
    pub keys: Vec<u32>,
    pub children: Vec<Node>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.keys.cmp(&other.keys)
    }
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

impl Nodeable<Node> for Node {
    fn insert(value: u32, into_node: Node) -> Option<Node> {
        let mut new_node = Node::default();
        if into_node.can_be_inserted(&value) {
            new_node.insert_keys_from_node_and_sort(value, into_node);
            new_node = Node::split_if_needed(new_node);
            new_node.children.sort();
            Some(new_node)
        } else if into_node.is_parent() {
            new_node = Node::insert_into_children(value, &into_node);
            new_node.children.sort();
            Some(new_node)
        } else {
            None
        }
    }

    fn count(node: Node) -> usize {
        if node.children.len() == 0 {
            node.keys.len()
        } else {
            node.children.iter().fold(node.keys.len(), |result, child| {
                let mut acc = result.clone();
                acc += Node::count(child.clone());
                acc
            })
        }
    }

    fn remove(value: u32, from_node: Node) -> Option<Node> {
        let mut keys = from_node.keys.clone();
        let mut new_node = from_node.clone();
        if let Some(index) = keys.iter().position(|&x| x == value) {
            keys.remove(index);
            new_node.keys = keys;
            Some(new_node)
        } else if new_node.children.len() > 0 {
            for child in Node::possible_children_for_removal(value, &new_node) {
                if let Some(removed_from) = Node::remove(value, child.clone()) {
                    if removed_from.keys.len() < removed_from.min_keys() {
                        new_node = Node::rebalance_after_removal(value, &new_node);
                    } else {
                        let index = new_node.children.iter().position(|c| c == child).unwrap();
                        new_node.children.remove(index);
                        new_node.children.insert(index, removed_from);
                    }
                    break;
                }
            }
            Some(new_node)
        } else {
            None
        }
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

    fn can_be_inserted(&self, value: &u32) -> bool {
        if self.keys.is_empty() {
            true
        } else if let Some(first) = self.keys.first() {
            value >= first && self.children.is_empty()
        } else {
            false
        }
    }

    fn is_parent(&self) -> bool {
        self.children.len() > 0
    }

    fn requires_splitting(&self) -> bool {
        self.keys.len() > self.max_keys
    }

    fn remaining_children(&self, index: usize) -> Vec<Node> {
        let mut remaining = self.children.clone();
        remaining.remove(index);
        remaining
    }

    fn insert_keys_from_node_and_sort(&mut self, value: u32, node: Node) {
        self.keys = node.keys.clone();
        self.keys.push(value);
        self.keys.sort();
    }

    fn insert_into_children(value: u32, node: &Node) -> Node {
        let mut new_node = node.clone();
        for (index, child) in node.children.iter().enumerate().rev() {
            if !child.can_be_inserted(&value) {
                continue;
            }
            if let Some(child_node) = Node::insert(value, child.clone()) {
                let child_was_split = child_node.is_parent();
                let remaining_children = new_node.remaining_children(index);
                if child_was_split {
                    new_node = Node::combine_parent_and_child(&node, &child_node);
                    new_node.children = child_node.children.clone();
                    new_node.children.extend(remaining_children);
                    new_node = Node::split_if_needed(new_node);
                } else {
                    new_node.children = vec![child_node];
                    new_node.children.extend(remaining_children);
                    println!(
                        "New child keys = {:?}",
                        new_node.children.last().unwrap().keys
                    );
                }
                break;
            };
        }
        new_node
    }

    fn combine_parent_and_child(parent_node: &Node, child_node: &Node) -> Node {
        let mut new_node = parent_node.clone();
        new_node.keys.extend(child_node.keys.clone());
        new_node
    }

    fn split_if_needed(node: Node) -> Node {
        if !node.requires_splitting() {
            return node;
        }
        let key_middle_index = node.keys.len() / 2;
        let middle_key = node.keys[key_middle_index].clone();
        let mut keys = node.keys.clone();
        let split_node = Node::new(keys.drain(..key_middle_index).collect());
        let right_node = Node::new(keys.drain(1..).collect());
        Node {
            keys: vec![middle_key],
            children: vec![split_node.clone(), right_node.clone()],
            ..Node::default()
        }
    }

    fn possible_children_for_removal(value: u32, from_node: &Node) -> &[Node] {
        let middle_index = from_node.keys.len() / 2;
        let middle_key = from_node.keys[middle_index];
        if value > middle_key {
            &from_node.children[middle_index + 1..]
        } else {
            &from_node.children[..middle_index + 1]
        }
    }

    fn gather_keys(node: &Node) -> Vec<u32> {
        let mut new_keys = node.keys.clone();
        if node.children.len() > 0 {
            let child_keys = node.children.iter().fold(vec![], |result, child| {
                let mut acc = result.clone();
                let child_keys = Node::gather_keys(child);
                acc.extend(child_keys);
                acc.sort();
                acc
            });
            new_keys.extend(child_keys);
            new_keys.sort();
        }
        new_keys
    }

    fn rebalance_after_removal(removed_key: u32, node: &Node) -> Node {
        let mut new_node = node.clone();
        let mut gathered_keys = Node::gather_keys(&new_node);
        let index = gathered_keys
            .iter()
            .position(|k| *k == removed_key)
            .unwrap();
        gathered_keys.remove(index);
        new_node.children = vec![];
        new_node.keys = vec![];
        gathered_keys.iter().for_each(|k| {
            new_node =
                Node::insert(*k, new_node.clone()).expect("There was an issue inserting the key.");
        });
        new_node
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
        if let Some(updated_node) = Node::insert(5, node) {
            assert_eq!(updated_node.keys.len(), 1);
            assert_eq!(*updated_node.keys.first().unwrap(), 5);
        }
    }

    #[test]
    fn test_multiple_inserts() {
        let mut node = Node::default();
        assert_eq!(node.keys.len(), 0);
        node = Node::insert(1, node.clone()).unwrap_or_default();
        node = Node::insert(2, node.clone()).unwrap_or_default();
        node = Node::insert(3, node.clone()).unwrap_or_default();
        node = Node::insert(4, node.clone()).unwrap_or_default();
        node = Node::insert(5, node.clone()).unwrap_or_default();
        assert_eq!(Node::count(node), 5);
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
        if let Some(inserted_node) = Node::insert(10, node) {
            assert_eq!(inserted_node.keys, vec![1, 2, 3]);
            assert_eq!(inserted_node.children.len(), 2);
            if let Some(child_1) = inserted_node.children.first() {
                assert_eq!(child_1.keys, vec![4, 5, 6]);
            }
            if let Some(last_child) = inserted_node.children.last() {
                assert_eq!(last_child.keys, vec![7, 8, 9, 10]);
            }
        }
    }

    #[test]
    fn test_splitting_leaf_nodes() {
        let node = Node {
            max_keys: 4,
            keys: vec![1, 2, 3, 4],
            children: vec![],
        };
        if let Some(new_parent_node) = Node::insert(5, node) {
            assert_eq!(new_parent_node.children.len(), 2);
            assert_eq!(new_parent_node.keys, vec![3]);
            if let Some(last_child) = new_parent_node.children.last() {
                assert_eq!(last_child.keys, vec![4, 5]);
            } else {
                panic!("Expected to find a child in the new parent node.")
            }
        }
    }

    #[test]
    fn test_count() {
        let mut node = Node {
            keys: vec![1, 2, 3],
            ..Default::default()
        };
        assert_eq!(Node::count(node.clone()), 3);
        node = Node::insert(4, node.clone()).unwrap();
        assert_eq!(Node::count(node.clone()), 4);
        let child_node_1 = Node {
            keys: vec![5, 6, 7],
            ..Default::default()
        };
        let child_node_2 = Node {
            keys: vec![8, 9, 10],
            ..Default::default()
        };
        node.children = vec![child_node_1, child_node_2];
        assert_eq!(Node::count(node), 10);
    }

    #[test]
    fn test_removing() {
        let node = Node {
            keys: vec![1, 2, 3, 4],
            ..Default::default()
        };
        if let Some(new_node) = Node::remove(4, node) {
            assert_eq!(new_node.keys, vec![1, 2, 3]);
        }
    }

    #[test]
    fn test_remove_from_children() {
        let child_1 = Node {
            keys: vec![0, 1, 2],
            ..Default::default()
        };
        let child_2 = Node {
            keys: vec![4, 5, 6],
            ..Default::default()
        };
        let node = Node {
            keys: vec![3],
            children: vec![child_1, child_2],
            ..Default::default()
        };
        if let Some(new_node) = Node::remove(4, node) {
            let last_child = new_node.children.last().unwrap();
            assert_eq!(last_child.keys, vec![5, 6]);
        } else {
            panic!("Could not remove element");
        }
    }

    #[test]
    fn test_removing_missing_value() {
        let node = Node {
            keys: vec![1, 2, 3, 4],
            ..Default::default()
        };
        let result = Node::remove(5, node);
        assert!(result.is_none());
    }

    #[test]
    fn test_removing_when_node_is_unbalanced() {
        let child_1 = Node {
            keys: vec![1, 2],
            ..Default::default()
        };
        let child_2 = Node {
            keys: vec![4, 5],
            ..Default::default()
        };
        let node = Node {
            keys: vec![3],
            children: vec![child_1, child_2],
            ..Default::default()
        };
        if let Some(result) = Node::remove(4, node) {
            assert_eq!(result.keys, vec![1, 2, 3, 5]);
            assert_eq!(result.children.len(), 0);
        } else {
            panic!("Error removing value")
        }
    }
}
