fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);
                        

    // println!("This is s: {s}"); <-- this wont work :D

    let x = 5; // x comes into scope

    makes_copy(x); 
    println!("This is x {x}")
    
    
    
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
}
