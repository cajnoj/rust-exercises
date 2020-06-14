const MAX_DEPTH: u128 = 10;

fn main() {
    println!("Hello, Fibonacci!");
    println!("Recursion result: {}", fibo_recursion());
    println!("Iteration result: {}", fibo_iteration());
}

fn fibo_recursion() -> u128 {
    _fibo_recursion(1, MAX_DEPTH)
}

fn _fibo_recursion(result: u128, i: u128) -> u128 {
    if i == 1 {
        result
    } else {
        i * _fibo_recursion(result, i-1)
    }
}

fn fibo_iteration() -> u128 {
    let mut result: u128 = 1;
    for i in 1..MAX_DEPTH+1 {
        result *= i;
    }
    result
}