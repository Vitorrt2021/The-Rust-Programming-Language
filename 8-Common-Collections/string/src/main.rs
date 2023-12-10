fn main() {
    create();
    update();
    read();
}


fn create(){
    let s = String::new();

    // the method also works on a literal directly:
    let s2 = "initial contents".to_string();

    let s3 = String::from("initial contents");
}

fn update(){
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s2 = String::from("lo");
    s2.push('l');

    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");
    let s5 = s3 + &s4; // note s3 has been moved here and can no longer be used
    let s6 = format!("{s4}-{s5}");
}

fn read(){
    for c in "Зд".chars() {
        println!("{c}");
    }
}