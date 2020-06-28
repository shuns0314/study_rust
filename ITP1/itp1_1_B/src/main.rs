use std::io;

fn main() {
    let mut x = String::new();

    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    
    let x: f32 = x.trim().parse()
        .expect("Please type a number");

    println!("{}", x.powi(3));
}
