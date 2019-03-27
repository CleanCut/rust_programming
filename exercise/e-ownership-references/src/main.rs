fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits.
    let mut arg: String = std::env::args()
        .collect::<Vec<String>>()
        .iter()
        .nth(1)
        .unwrap_or_else(|| {
            println!("Please supply an argument to this program.");
            std::process::exit(-1);
        }).to_owned();

    // 1. Write a function "inspect()" that takes a reference to arg and prints
    //    whether arg is plural or singular.  Then uncomment and run this code.
    //    Hint: use arg.ends_with("s")
    //
    inspect(&arg);

    // 2. Write a function "change()" that takes a mutable reference to arg
    //    and adds an "s" to the string if it doesn't already end with "s".
    //    Then uncomment and run this code.  Hint: use arg.push_str("s")
    //
    change(&mut arg);
    println!("I have many {}", arg);

    // 3. Write a function "eat" that consumes arg and returns a bool
    //    indicating whether or not arg contains an "a" AND starts with a "b".
    //    Then uncomment and run this code.
    //    Hint 1: use arg.contains("a")
    //    Hint 2: && is the boolean AND operator
    //
    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    // Try running this program with "apple", "banana", and "grapes"  :-)
}


fn eat(arg: String) -> bool {
    if arg.contains("a") && arg.starts_with("b") {
        true
    } else {
        false
    }
}

fn change(arg: &mut String) {
    if !arg.ends_with("s") {
        arg.push_str("s");
    }
}

fn inspect(arg: &String) {
    if arg.ends_with("s") {
        println!("plural");
    } else {
        println!("singular");
    }
}