use std::io;
use std::ascii::AsciiExt as Ascii;

fn main() {
    let mut s: String = String::new();
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    println!("Enter word to piggify: ");
    match io::stdin().read_line(&mut s) {
        Ok(_) => (),
        Err(error) => panic!("Error: {}", error),
    }

    let s = s.trim();
    let mut r = String::new();
    let mut tail: String = String::new();

    for (i, c) in s.chars().enumerate() {
        match i {
            0 => {
                let lc = Ascii::to_ascii_lowercase(&c);
                if vowels.contains(&lc) {
                    tail += "-hay";
                } else {
                    tail.push('-');
                    tail.push(lc);
                    tail += "ay";
                }
            },
            1 => r.push(Ascii::to_ascii_uppercase(&c)),
            _ => r.push(c),
        }
    }
    
    r += &tail;

    println!("{}", r);
}
