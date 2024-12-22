mod bst;

use bst::Bst;

fn main() {
    println!("Hello, here is LiuYi's algorithm exercise!");
    println!("");

    println!("1. BST Test");
    bst_test();
}

fn bst_test() {
    let mut bst = Bst::<u64>::new();

    let test_vector = vec![4, 2, 1, 3, 6, 5, 7];
    println!("test vector {:?}", test_vector);
    println!("");
    for ele in test_vector {
        bst.insert(ele);
    }

    print!(" pre order traversal: ");
    bst.preorder_traversal();
    print!("  in order traversal: ");
    bst.inorder_traversal();
    print!("post order traversal: ");
    bst.postorder_traversal();
    println!("       find max data: {}", bst.max().unwrap());
    println!("       find min data: {}", bst.min().unwrap());
    println!("    search 10 in bst: {}", bst.search(10));
    println!("    search  6 in bst: {}", bst.search(6));
}
