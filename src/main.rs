mod bst;

use bst::Bst;

fn main() {
    println!("Hello, here is LiuYi's algorithm exercise!");
    let test_vector = vec![4, 2, 1, 3, 6, 5, 7];
    println!("test vector {:?}", test_vector);
    println!("");

    println!("1. BST Test");
    bst_test(test_vector);
}

fn bst_test(test_vector: Vec<u64>) {
    let mut bst = Bst::<u64>::new();

    for ele in test_vector {
        bst.insert(ele);
    }

    print!(" pre order traversal: ");
    bst.preorder_traversal();
    print!("  in order traversal: ");
    bst.inorder_traversal();
    print!("post order traversal: ");
    bst.postorder_traversal();
}
