fn main() {
    // 'n' will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        }
        else if n % 3 == 0 {
            println!("fizz");
        }
        else if n % 5 == 0 {
            println!("buzz");
        }
        else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),

        }
    }
    
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
   
    // eventually try to manipulate entries
    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => {
                println!("There is a rustacean among us! Mark him!");
                *name = "Ferris the Rustacean";
            },
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);
}
