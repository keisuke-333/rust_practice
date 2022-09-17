fn fizzbuzz(end: i32) {
    let mut num = 1;
    while num <= end {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", num);
        }
        num += 1;
    }
}

fn main() {
    fizzbuzz(30);
}
