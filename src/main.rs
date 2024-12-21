use rand::Rng;

mod bst;

use bst::Bst;

fn main() {
    println!("Hello, here is LiuYi's algorithm exercise!");
    let mut rng = rand::thread_rng();
    let test_vector = (0..18).map(|_| rng.gen_range(0..=1000)).collect();
    bst_test(test_vector);
}

fn bst_test(test_vector: Vec<u64>) {
    let mut bst = Bst::<u64>::new();

    for ele in test_vector {
        bst.insert(ele);
    }
}