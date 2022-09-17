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

fn fizzbuzz2(end: i32) {
    let nums = 1..=end;
    for num in nums {
        match num % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", num),
        }
    }
}

fn fizzbuzz3(end: i32) {
    let nums = 1..=end;
    for num in nums {
        match (num % 3, num % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", num),
        }
    }
}

fn main() {
    fizzbuzz(30);
    fizzbuzz2(30);
    fizzbuzz3(30);
}
