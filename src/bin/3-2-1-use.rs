#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}
 
enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use Status::{Poor,Rich};
    // Automatically `use` each name inside `Work`.
    use Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Ẁork::Civilian`.
    let work = Civilian;

    match status {
        Poor => println!("The rich have lots of money!"),
        Rich => println!("The poor have no money ..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
