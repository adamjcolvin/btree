use btree::BTree;

fn main() {
    let tree = BTree::new();
    println!("The number of items in the tree is {}", tree.count());
}
