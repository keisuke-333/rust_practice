fn need_even(num: i32) -> Result<i32, &'static str> {
    if num % 2 == 0 {
        Ok(num)
    } else {
        Err("引数は偶数にしてください")
    }
}

fn double_even(num: i32) -> Result<i32, &'static str> {
    let x = need_even(num)?;
    Ok(x * 2)
}

fn main() {
    match double_even(10) {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err),
    }
}
