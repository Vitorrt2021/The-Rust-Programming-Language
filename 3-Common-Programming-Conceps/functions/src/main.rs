use std::io;


fn main() {
    calculate();

    // let sum1: String = sum(1,2).to_string();
    // println!("{}", sum1);

    /*
        Statements are instructions that perform some action and do not return a value.
        Expressions evaluate to a resultant value. Let’s look at some examples.
    */

    // Statements 
    let big: i32 = 255;

    // Expressions
    let y = {
        let x = 3;
        x + 1
    };

    /*
        If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.
    */
    {
        let n1 = 12;
        n1 + 2 
    };
}

fn sum(a: i64, b: i64) -> i64 {
    a + b
}

fn multiply(a: i64, b: i64) -> i64{
    a * b
}

fn divide(a: i64, b: i64) -> i64{
    a / b
}

fn subtract(a: i64, b: i64) -> i64{
    a - b
}

fn calculate(){
    loop {
        let mut n1 = String::new();
        let mut n2 = String::new();
        let mut operation = String::new();

        println!("Input value 1");
        io::stdin()
            .read_line(&mut n1)
            .expect("Failed to read line");
        let n1: i64 = match n1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        println!("Input value 2");
        io::stdin()
            .read_line(&mut n2)
            .expect("Failed to read line");
        let n2: i64 = match n2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        println!("Operation (mul, div, sum, sub)");
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        let operation = operation.trim();

        match operation {
            "mul" => println!("Result: {}", multiply(n1, n2)),
            "div" => println!("Result: {}", divide(n1, n2)),
            "sum" => println!("Result: {}", sum(n1, n2)),
            "sub" => println!("Result: {}", subtract(n1, n2)),
            _ => println!("Invalid operation"),
        }
    }
}