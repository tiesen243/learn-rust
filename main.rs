fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    use std::io::{stdin, stdout, Write};

    let a: i32;
    let b: i32;

    print!("Enter first number: ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    a = input.trim().parse().unwrap();
    
    print!("Enter second number: ");
    stdout().flush().unwrap();
    input.clear();
    stdin().read_line(&mut input).unwrap();
    b = input.trim().parse().unwrap();

    println!("Sum of {} and {} is {}", a, b, sum(a, b));
}
