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
    println!("{s2}");
    // Error -> borrow of moved value: `s1` value borrowed here after move
    // s1 was moved into s2 != shallow copy

    // Scope and assignment
    let mut reassign = String::from("hello");
    reassign = String::from("ahoy");
    println!("{reassign}, world!");

    // Clone
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // Stack only
    // Simple types that have known size at compile time are stored on the stack
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Ownership and functions
    let test1 = String::from("hello"); // s comes into scope

    takes_ownership(test1); // s's value moves into the function...
                            // ... and so is no longer valid here

    let h = 5; // x comes into scope

    makes_copy(h); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
                   // println!("{test1}")
                   // Error -> borrow of moved value: `test1` value borrowed here after move

    // References and Borrowing
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // Mutable references
    let mut s = String::from("hello");
    change(&mut s);
    // Error -> cannot borrow `s` as mutable because it is not declared as mutable
    // To avoid having multiple borrowings at once
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    // Users of an immutable reference don’t expect the value to suddenly change out from under them!
    // However, multiple immutable references are allowed because no one who is just reading
    // the data has the ability to affect anyone else’s reading of the data.
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
    // The scopes of the immutable references r1 and r2 end after the println!
    // where they are last used, which is before the mutable reference r3 is created.
    // These scopes don’t overlap, so this code is allowed: the compiler can tell that the reference
    // is no longer being used at a point before the end of the scope.
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) -> usize {
    // We call the action of creating a reference borrowing
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
