use std::collections::HashMap;

fn main() {
    create();
    read();
}

fn create(){
    let mut scores = HashMap::new();

    // Override the value if already exists
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // if not exists, create
    scores.entry(String::from("Yellow")).or_insert(50);
}

fn read(){
    let scores: HashMap<String, i32> = [ 
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
    ].iter().cloned().collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}