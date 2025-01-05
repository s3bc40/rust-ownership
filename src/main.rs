// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`

    // “bind the value 5 to x; then make a copy of the value in x and bind it to y.”
    let x = 5;
    let y = x;

    // When we assign s1 to s2, the String data is copied, meaning we copy the pointer,
    // the length, and the capacity that are on the stack
    let s1 = String::from("hello");
    let s2 = s1;
    // s2 and s1 go out of scope, they will both try to free the same memory.
    // This is known as a double free error
    println!("{s1}");
    // Error -> borrow of moved value: `s1` value borrowed here after move
    // s1 was moved into s2 != shallow copy
}
