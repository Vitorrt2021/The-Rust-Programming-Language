use macros::vecDuplicate;
use macros::HelloMacro;
use hello_macro_derive::*;

#[derive(HelloMacro)]
struct Pancakes;


fn main() {
    let v: Vec<u32> = vec![1, 2, 3];

    let v2: Vec<u32> = vecDuplicate![1,2,3];

    v2.iter().for_each(|x| println!("{}", x));


    Pancakes::hello_macro();
}
