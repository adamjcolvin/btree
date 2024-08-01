type RcNode = Box<Node>;

#[derive(Clone, Debug)]
pub struct Node {
    pub max_keys: usize,
    pub keys: Vec<u32>,
    pub children: Vec<RcNode>,
    pub parent: Option<RcNode>,
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

    pub fn insert(value: u32, into_node: RcNode) -> RcNode {
        let node = into_node.clone();
        let mut keys = node.keys.clone();
        let mut children: Vec<RcNode> = vec![];
        if node.children.len() == 0 {
            keys.push(value);
            keys.sort();
            if keys.len() > node.max_keys {
                //Node::split()
            }
        } else {
            children.extend(Node::insert_into_children(value, node.children.clone()));
        }
        Box::new(Node {
            keys,
            children,
            ..Default::default()
        })
    }

    fn insert_into_children(value: u32, children: Vec<RcNode>) -> Vec<RcNode> {
        let mut new_children: Vec<RcNode> = vec![];
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

    fn split(node: Node) -> RcNode {
        let key_middle_index = node.keys.len() / 2;
        let middle_key = node.keys[key_middle_index].clone();
        let mut parent = node.parent.clone();

        if let Some(parent) = node.parent {
            //Insert into existing parent
        } else {
            //Create new parent
        }
        Box::new(Node {
            keys: vec![middle_key],
            parent,
            ..Default::default()
        })
    }

    // fn split(&mut self) {
    //     let key_middle_index = self.keys.len() / 2;
    //     let middle_key = self.keys[key_middle_index].clone();

    //     if let Some(parent) = &self.parent {
    //         if let Some(parent_node) = parent.upgrade() {
    //             let mut new_node = Node::new();
    //             new_node.keys.extend(self.keys.drain(..key_middle_index));
    //             let mut parent_ref = parent_node.borrow_mut();
    //             let pos = parent_ref
    //                 .children
    //                 .iter()
    //                 .position(|child| Rc::ptr_eq(child, &Rc::new(RefCell::new(*self))))
    //                 .unwrap();
    //             parent_ref
    //                 .children
    //                 .insert(pos + 1, Rc::new(RefCell::new(new_node)));
    //             parent_ref.insert(middle_key);
    //         }
    //     } else {
    //         let mut left_node = Node::new();
    //         left_node.keys.extend(self.keys.drain(..key_middle_index));
    //         let mut right_node = Node::new();
    //         right_node.keys.extend(self.keys.drain(..));
    //         let mut new_root = Node::default();
    //         new_root.keys.push(middle_key);
    //         new_root.children.push(Rc::new(RefCell::new(left_node)));
    //         new_root.children.push(Rc::new(RefCell::new(right_node)));
    //         for child in new_root.children.iter_mut() {
    //             //Set the parent of each child node to be self.
    //             //We'll then set the keys and children on self from the new root.
    //             child.borrow_mut().parent = Some(Rc::downgrade(&Rc::new(RefCell::new(*self))));
    //         }
    //         self.keys = new_root.keys.clone();
    //         self.children = new_root.children.clone();
    //     }
    // }
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
        // assert!(split_node.borrow().parent.is_some());
        // let parent = node.parent;
        // assert_eq!(parent.children.len(), 2);
        // let expected_parent_key = parent.keys.last().unwrap();
        // assert_eq!(*expected_parent_key, 3);
        // let expected_child_key = parent.children.last().unwrap().keys.last().unwrap();
        // assert_eq!(*expected_child_key, 5);
    }

    #[test]
    fn test_number_of_child_nodes() {}
}
