use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("score: {score}");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash Maps and Ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("map: {:?}", map);  // map: {"Favorite color": "Blue"}

    // Overwriting a Value

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("scores: {:?}", scores);  // scores: {"Blue": 25}

    // Adding a Key and Value Only if a Key Isn't Present

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("scores: {:?}", scores);  // scores: {"Yellow": 50, "Blue": 10}

    // Updating a Value Based on the Old Value

    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map: {:?}", map);  // map: {"Hello": 1, "world": 2, "wonderful": 1}
}
