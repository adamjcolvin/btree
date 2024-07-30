use btree::BTree;

fn main() {
    let tree = BTree::new(vec![]);
    println!("The number of items in the tree is {}", tree.count());
}
