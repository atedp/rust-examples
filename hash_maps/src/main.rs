use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();

    let teams = vec![String::from("Orange"), String::from("Red")];
    let initial_scores = vec![20, 40];

    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);


    println!("{:#?}", scores);
}