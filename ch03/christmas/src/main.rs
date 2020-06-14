const OPENING: &str = "On the twelfth day of Christmas\nMy true love gave to me";
const LINES: [&str; 12] = [
    "12 drummers drumming",
    "Eleven pipers piping",
    "Ten lords a leaping",
    "Nine ladies dancing",
    "Eight maids a milking",
    "Seven swans a swimming",
    "Six geese a laying",
    "Five gold rings, badam-pam-pam",
    "Four calling birds",
    "Three French hens",
    "Two turtle doves",
    "A partridge in a pear tree",
];

fn main() {
    for i in (0..LINES.len()).rev() {
        println!("\n{}", OPENING);
        for j in i..LINES.len() {
            if j == LINES.len() - 1 && i != j {
                print!("And ");
            }
            println!("{}", LINES[j]);
        }
    }
}
