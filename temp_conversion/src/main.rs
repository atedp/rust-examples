use std::io;

fn main() {
    println!("Welcome to the Temperature Converter!");
    println!("Please select 1 to convert from F to C, or 2 to convert from C to F");

    let mut direction = String::new();

    io::stdin().read_line(&mut direction)
    .expect("Failed to read input");

    let direction: u32 = direction.trim().parse()
    .expect("Please type a number!");

    match 

}
