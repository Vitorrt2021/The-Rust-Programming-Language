fn main() {
    /* 
     
     A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters 
    
    */
    let int8: i8 = 8;
    let uint64: u64= 123;
    let big_int: u128 = 99_999_999_999_999_999_999;
    println!("integers {}, {}, {}", int8, uint64, big_int);

    let f8: f32 = 1.1;
    let f_big: f64 = 122_111.231212;
    println!("float {}, {}", f8, f_big);

    let boolean: bool = false;
    println!("boolean {}", boolean,);

    let c: char = 'c';
    println!("char {}", c );

    /*
        Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
    */

    let tup: (u128, u128, char) = (0, 2, 'a');
    let (tup0, tup1, _tup2) = tup;
    println!("tuple {}, {}, {}", tup0, tup1, tup.2);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array {}, {}", arr[0], arr[1]);
}
