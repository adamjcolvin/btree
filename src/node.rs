use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

type RcNode = Box<Node>;
type ParentNode = Option<Weak<RefCell<Node>>>;

#[derive(Clone, Debug)]
pub struct Node {
    pub max_keys: usize,
    pub keys: Vec<u32>,
    pub children: Vec<RcNode>,
    pub parent: ParentNode,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            max_keys: 4,
            keys: vec![],
            children: vec![],
            parent: None,
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

    pub fn insert(value: u32, into_node: RcNode) -> ParentNode {
        let node = into_node.clone();
        let mut new_node = Node::default();
        new_node.keys = node.keys.clone();
        new_node.children = vec![];
        if node.children.len() == 0 {
            new_node.keys.push(value);
            new_node.keys.sort();
            if new_node.keys.len() > node.max_keys {
                Node::split(Box::new(new_node))
            } else {
                let new_parent = Rc::new(RefCell::new(new_node));
                Some(Rc::downgrade(&new_parent))
            }
        } else {
            new_node
                .children
                .extend(Node::insert_into_children(value, node.children.clone()));
            let new_parent = Rc::new(RefCell::new(new_node));
            Some(Rc::downgrade(&new_parent))
        }
    }

    fn insert_into_children(value: u32, children: Vec<RcNode>) -> Vec<RcNode> {
        let mut new_children: Vec<RcNode> = vec![];
        for (index, child) in children.iter().enumerate().rev() {
            if let Some(lowest_key) = child.keys.first() {
                if value >= *lowest_key {
                    let new_child =
                        Node::child_node_from_parent(Node::insert(value, child.clone()));
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

    fn child_node_from_parent(parent: ParentNode) -> RcNode {
        if let Some(parent_node) = parent {
            if let Some(rc_child) = parent_node.upgrade() {
                let ref_child = rc_child.borrow();
                let cloned_child = ref_child.clone();
                Box::new(cloned_child)
            } else {
                Box::new(Node::default())
            }
        } else {
            Box::new(Node::default())
        }
    }

    fn split(node: RcNode) -> ParentNode {
        let key_middle_index = node.keys.len() / 2;
        let middle_key = node.keys[key_middle_index].clone();
        let mut keys = node.keys.clone();
        let split_node = Box::new(Node::new(keys.drain(..key_middle_index).collect()));
        let right_node = Box::new(Node::new(keys.drain(..).collect()));
        let new_parent = Rc::new(RefCell::new(Node {
            keys: vec![middle_key],
            children: vec![split_node.clone(), right_node.clone()],
            parent: node.parent.clone(),
            ..Node::default()
        }));

        for child in new_parent.borrow_mut().children.iter_mut() {
            child.parent = Some(Rc::downgrade(&new_parent));
        }

        Some(Rc::downgrade(&new_parent))
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
        let node = Box::new(node);
        let updated_node = Node::insert(5, node);
        assert_eq!(updated_node.keys.len(), 1);
    }

    #[test]
    fn test_insert_into_child_node() {
        let child_1 = Box::new(Node::new(vec![4, 5, 6]));
        let child_2 = Box::new(Node::new(vec![7, 8, 9]));
        let node = Node {
            max_keys: 4,
            keys: vec![1, 2, 3],
            children: vec![child_1, child_2],
            ..Default::default()
        };
        let inserted_node = Node::insert(10, Box::new(node));
        assert_eq!(inserted_node.keys, vec![1, 2, 3]);
        assert_eq!(inserted_node.children.len(), 2);
        if let Some(child_1) = inserted_node.children.first() {
            assert_eq!(child_1.keys, vec![4, 5, 6]);
        };
        if let Some(child_2) = inserted_node.children.last() {
            assert_eq!(child_2.keys, vec![7, 8, 9, 10]);
        };
    }

    #[test]
    fn test_splitting_leaf_nodes() {
        let node = Node {
            max_keys: 4,
            keys: vec![1, 2, 3, 4],
            children: vec![],
            parent: None,
        };
        assert!(node.parent.is_none());
        let split_node = Node::insert(5, Box::new(node));
        assert!(split_node.parent.is_some());
        // assert_eq!(split_node.parent.unwrap().children.len(), 2);
        // let expected_parent_key = parent.keys.last().unwrap();
        // assert_eq!(*expected_parent_key, 3);
        // let expected_child_key = parent.children.last().unwrap().keys.last().unwrap();
        // assert_eq!(*expected_child_key, 5);
    }

    #[test]
    fn test_number_of_child_nodes() {}
}
