// A function which takes a closure as an argument and calls it.
fn apply<F>(f: F) where 
    // The closure takes no input and returns nothing
    F: FnOnce() {
       f();
}
    
fn apply_to_3<F>(f: F) -> i32 where 
    // The closure takes an 'i32' and returns an 'i32'
    F: Fn(i32) -> i32 {
        f(3)
}
