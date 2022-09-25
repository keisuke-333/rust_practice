mod test_module;

use rand::Rng;
use test_module::*;

fn main() {
    let rand_number = rand::thread_rng().gen_range(1..101);
    println!("{}", rand_number);
    sub1::test_fn();
    sub2::test_fn();
}
