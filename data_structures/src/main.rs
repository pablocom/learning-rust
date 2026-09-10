use std::rc::Rc;

use data_structures::tree_node::TreeNode;

fn main() {
    let leaf_node = TreeNode::new(3);

    print_ref_counts("leaf (initial)", &leaf_node);

    {
        let branch_node = TreeNode::new(5);
        branch_node.add_child(Rc::clone(&leaf_node));

        print_ref_counts("branch (inside scope)", &branch_node);
        print_ref_counts("leaf (linked to branch)", &leaf_node);
    }

    let parent_upgrade = leaf_node.parent.borrow().upgrade();
    println!("leaf parent after scope exit: {parent_upgrade:?}");

    print_ref_counts("leaf (after branch dropped)", &leaf_node);
}

fn print_ref_counts<T>(label: &str, node: &Rc<TreeNode<T>>) {
    println!(
        "[{label}] strong count: {}, weak count: {}",
        Rc::strong_count(node),
        Rc::weak_count(node)
    );
}
