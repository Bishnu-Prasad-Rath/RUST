use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Bishnu"), 10);
    scores.insert(String::from("Kalyani"), 20);

    // scores.insert(String::from("Bishnu"), 100); //Overwrite the value

    scores.entry(String::from("Balraj")).or_insert(50); //This line is ssaying that if the Balraj key and 50 value does not exist thne create it

    let score = scores.get(&String::from("blue")).copied().unwrap_or(0);

    for (key, value) in scores {
        println!("{:?} => {:?}", key, value);
    }

    let text = "This is a random sentence for looping";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        println!("Word = {word}");
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map {
        println!("{:?} => {:?}", key, value);
    }

    println!("Score = {score}");
}
