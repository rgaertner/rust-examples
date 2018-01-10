fn main() {
    // The compiler warns about unused variable bindings; these warnings can 
    // be silenced by prefixing the variable name with an underscore
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation {}", mutable_binding);

    // Error!
    // _immutable_binding += 1;
}
