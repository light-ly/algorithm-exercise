mod bst;

use std::time::Instant;

use bst::Bst;

fn main() {
    println!("Hello, here is LiuYi's algorithm exercise!");
    println!("");

    println!("1. BST Test");
    bst_test();
}

fn bst_test() {

    let timer = Instant::now();

    let mut bst = Bst::<u64>::new();

    let test_vector = vec![4, 2, 1, 3, 6, 5, 7];
    println!("test vector {:?}", test_vector);
    println!("");

    println!("init complete! elapsed time: {} ns", timer.elapsed().as_nanos());
    println!("");
    let timer = Instant::now();

    for ele in test_vector {
        bst.insert(ele);
    }

    println!("insert complete! elapsed time: {} ns", timer.elapsed().as_nanos());
    println!("");
    println!("see bst {:#?}", bst);
    println!("");
    let timer = Instant::now();

    print!(" pre order traversal: ");
    bst.preorder_traversal();
    print!("  in order traversal: ");
    bst.inorder_traversal();
    print!("post order traversal: ");
    bst.postorder_traversal();
    println!("       find max data: {}", bst.max().unwrap().data());
    println!("       find min data: {}", bst.min().unwrap().data());
    println!("    search 10 in bst: {}", bst.search(10).is_some());
    println!("    rank of 4 in bst: {}", bst.rank(4));
    println!("    rank of 6 in bst: {}", bst.rank(6));

    println!("basic test complete! elapsed time: {} ns", timer.elapsed().as_nanos());
    println!("");
    let timer = Instant::now();

    vec![5, 2, 6].iter().for_each(|&i| {
        println!("    search  {i} in bst: {}", bst.search(i).is_some());
        println!("    remove  {i} in bst");
        bst.remove(i);
        println!("    search  {i} in bst: {}", bst.search(i).is_some());
        print!(" pre order traversal: ");
        bst.preorder_traversal();
        print!("  in order traversal: ");
        bst.inorder_traversal();
        print!("post order traversal: ");
        bst.postorder_traversal();
        println!("    rank of 4 in bst: {}", bst.rank(4));
        println!("   get rank 4 in bst: {}", bst.get_with_rank(4).unwrap().data());
        println!("");
    });

    println!("insert complete! elapsed time: {} ns", timer.elapsed().as_nanos());
    println!("");
}
