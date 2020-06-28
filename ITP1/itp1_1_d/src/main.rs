use std::io;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line!");
    let x: i32 = x.trim().parse().expect("Failed to parse input!");

    let h: i32 = x / 3600;
    let m: i32 = x % 3600 / 60;
    let s: i32 = x % 3600 % 60;

    println!("{}:{}:{}", h, m, s);
}
