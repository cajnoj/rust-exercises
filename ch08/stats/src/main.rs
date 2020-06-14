use std::io;
use std::collections::HashMap;

fn main() {
    println!("Enter list of numbers. Terminate by pressing enter.\n");

    let mut v: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    let mut map = HashMap::new();
    let mut mode: usize = 0;

    // Input numbers
    loop {
        println!("Enter number: ");

        let mut num = String::new();
        match io::stdin().read_line(&mut num) {
            Ok(_) => (),
            Err(error) => println!("Error: {}", error),
        }

        let num = num.trim();
        if num.is_empty() {
            break;
        }

        let val: i32 = match num.parse() {
            Ok(x) => x,
            Err(error) => panic!("Error: {}", error),
        };

        // Add value to vector
        &v.push(val);

        // Compute sum
        sum += val;

        // Compute mode
        let count = map.entry(val).or_insert(0);
        *count += 1;
        if *count > mode {
            mode = *count;
        }
    }
    v.sort();

    // Compute mean
    let mean = (sum as f32) / (v.len() as f32);
    println!("Mean: {}", mean);

    // Compute median
    for (i, val) in v.iter().enumerate() {
        if i >= v.len() / 2 {
            println!("Median: {}", val);
            break;
        }
    }

    // Print mode
    println!("Mode: {}", mode);
}