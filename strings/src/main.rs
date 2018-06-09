fn main() {

    let len = String::from("Hola").len();

    println!("len = {}", len);

    let hello = "Здравствуйте";
    let s = &hello[..4];

    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    for c2 in hello.chars() {
        println!("{}", c2);
    }
}
