fn main() {
    type_alias();
}


fn type_alias() {
    type Kilometers = i32;

    let x: i32 = 2;
    let y: Kilometers = 24;

    println!("x + y: {}", x + y);
}
