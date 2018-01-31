// Fn: the closure captures by reference (&T)
// FnMut: the closure captures by mutable reference (&mut T)
// FnOnce: the closure captures by value (T)
//
// A function which takes a closure as an argument and calls it.
fn apply<F>(f: F, string: String) where 
    // The closure takes no input and returns nothing
    F: FnOnce(String) {
        // try changing to Fn or FnMut
       f(string);
}
    
fn apply_to_3<F>(f: F) -> i32 where 
    // The closure takes an 'i32' and returns an 'i32'
    F: Fn(i32) -> i32 {
        f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";

    // A non-copy type
    // 'to_owned' creates owned data from borrowed one
    let farewell = "goodbye".to_owned();

    // Capture 2 variables: 'greeting' by reference and 
    // 'farewell' by value.
    let diary = | mut farewell: String| {
        // 'greeting' is by reference: requires 'Fn'.
        println!("I said {}.", greeting);

        // Mutation forces 'farewell' to be captured by 
        // mutable reference. Now requires 'FnMut'.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("now I can sleep. zzzz");

        // Manually calling drop forces 'farewell' to 
        // be captured by value. Now requires 'FnOnce'.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    //apply(&diary);
    apply(diary,farewell);

    //println!("{}", farewell);
    // 'double' satisfies 'apply_to_3's trait bound
    let double = |x| x * 2;
    
    println!("3 doubled: {}", apply_to_3(double));
}
