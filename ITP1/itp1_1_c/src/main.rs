use std::io;

fn main() {
    let mut x = String::new();

    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    
    let vec: Vec<&str> = x.trim().split_whitespace().collect();
    let x: i32 = vec[0].parse().expect("Failed to parse");
    let y : i32 = vec[1].parse().expect("Failed to parse");

    println!("{} {}", x*y, (x+y)*2);
}
