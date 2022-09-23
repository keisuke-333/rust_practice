use rand::Rng;

fn main() {
    let rand_number = rand::thread_rng().gen_range(1..101);
    println!("{}", rand_number);
}
