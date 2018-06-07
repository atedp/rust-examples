use std::io;
use std::io::prelude::*;

fn main() {
    println!("Hello!");
    println!("Which fibonacci number would you like to find?");

    let mut pick = String::new();

    io::stdin().read_line(&mut pick)
    .expect("Failed to read input");

    let pick: u32 = match pick.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    println!("The {}th Fibonacci number is: ", pick);
    fibonacci(pick);
}

fn fibonacci(n: u32) {

    let mut a = 0;
    let mut b = 1;
    let mut c;
    let i: u32;

    if n == 0 {
        println!("{}", a);
    } else if n == 1 {
        println!("{}", b);
    } else { 
        for i in (2..n) {
        c = a + b;
        a = b;
        b = c;
    }
    println!("{}", b);
    pause();
    }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stout = io::stdout();

    write!(stout, "Press any key to continue...").unwrap();
    stout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}