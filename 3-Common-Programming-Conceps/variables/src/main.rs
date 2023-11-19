fn main() {
    let x = 5;
    println!("Immutable value  x = {x}");

    {
        let x = x + 1;
        println!("shadowed value of x = {}", x);
    }
    println!("x continue = {}", x);

    let mut mutable = false;
    mutable = true;
    println!("mutable {mutable}!");

    const CONSTANT_VALUE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
}
