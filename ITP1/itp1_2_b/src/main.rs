use std::io;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line!");
    let vec: Vec<&str> = x.split_whitespace().collect();
    let a: u8 = vec[0].parse().expect("Failed to parse input!");
    let b: u8 = vec[1].parse().expect("Failed to parse input!");
    let c: u8 = vec[2].parse().expect("Failed to parse input!");

    match (a < b) & (b < c) {
        true => println!("Yes"),
        false => println!("No"),
    }
}
