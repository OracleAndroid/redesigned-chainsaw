use std::io;
use std::io::stdin;

fn getpath() {
    println!("Hello, please input a path containing Images to sort:");
    let mut path = String::new();

    io::stdin().read_line(&mut path)
        .expect("Failed to read path");

    

    }
}

fn main() {
    getpath();
}