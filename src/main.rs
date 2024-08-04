use btree::BTree;
use btree::Node;

fn main() {
    let tree: BTree<Node> = BTree::default();
    println!("The number of items in the tree is {}", tree.count());
}
