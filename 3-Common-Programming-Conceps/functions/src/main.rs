fn main() {
    hello_world();

    let sum: String = sum(1,2).to_string();
    println!("{}", sum);

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

fn hello_world(){
    println!("Hello, world!");
}


fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn multiply(a: i32, b: i32) -> i32{
    a * b
}