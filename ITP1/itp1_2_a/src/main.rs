use std::io;
use std::cmp::Ordering;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line!");
    let vec: Vec<&str> = x.trim().split_whitespace().collect();

    let a: i32 = vec[0].parse().expect("Failed to parse input");
    let b: i32 = vec[1].parse().expect("Failed to parse input");

    match a.cmp(&b) {
        Ordering::Less => println!("a < b"),
        Ordering::Greater => println!("a > b"),
        Ordering::Equal => println!("a == b"),
    }
}
