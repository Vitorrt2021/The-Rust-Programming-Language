fn main() {
    /* 
    Programming languages like Rust, a systems programming language, require understanding the stack and heap for effective memory management.
    The stack stores values in a last in, first out (LIFO) order, similar to a stack of plates. It is fast and suitable for data with a known, fixed size.
    Data with an unknown size or a size that might change is stored on the heap, which is less organized. Allocating on the heap involves requesting a specific amount of space, and a pointer to the allocated space is returned.
    Pushing values onto the stack is faster than allocating on the heap because the location for new data is always at the top of the stack.
    Accessing data on the heap is slower than accessing data on the stack due to the need to follow a pointer to get to the actual data.
    Processors are more efficient when working with data that is close to each other (on the stack) rather than scattered (on the heap).
    Function calls involve pushing values onto the stack, including potentially pointers to data on the heap. After the function completes, these values are popped off the stack.
    Ownership in Rust addresses issues such as tracking heap data usage, minimizing duplicate data on the heap, and cleaning up unused data to prevent running out of space.
    Understanding ownership helps manage heap data efficiently, and once grasped, the distinction between the stack and heap becomes less prominent in daily programming considerations.
    */
    println!("Hello, world!");


    /*
    Each value in Rust has an owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped.
     */

     let mut s = String::from("hello");
     s.push_str(", world!"); // push_str() appends a literal to a String

     println!("{}", s); // This will print `hello, world!`

}
