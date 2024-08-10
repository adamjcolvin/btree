use btree::BTree;

#[test]
fn test_empty_btree() {
    assert_eq!((BTree::default() as BTree).count(), 0);
}

#[test]
fn test_tree_count() {
    let mut tree: BTree = BTree::default();
    tree.bulk_insert(vec![1, 2, 3, 4]);
    assert_eq!(tree.count(), 4);
}

#[test]
fn test_inserting_element() {
    let mut btree: BTree = BTree::default();
    btree.insert(1);
    assert_eq!(btree.count(), 1);
}

#[test]
fn test_inserting_multiple_elements() {
    let mut btree: BTree = BTree::default();
    btree.bulk_insert(vec![1, 2, 3, 4, 5]);
    assert_eq!(btree.count(), 5);
}

#[test]
fn test_removing_element() {
    let mut btree: BTree = BTree::default();
    btree.bulk_insert(vec![1, 2, 3, 4, 5]);
    assert_eq!(btree.count(), 5);
    btree.remove(5);
    assert_eq!(btree.count(), 4);
}

#[test]
fn test_removing_multiple_elements() {
    let mut btree: BTree = BTree::default();
    btree.bulk_insert(vec![1, 2, 3, 4, 5]);
    assert_eq!(btree.count(), 5);
    btree.bulk_remove(vec![2, 3, 4]);
    assert_eq!(btree.count(), 2);
}

#[test]
fn test_contains() {
    let mut btree: BTree = BTree::default();
    btree.bulk_insert(vec![1, 2, 3, 4, 5]);
    assert!(btree.contains(3));
    btree.remove(3);
    assert!(!btree.contains(3));
}
