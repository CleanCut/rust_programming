#![allow(dead_code, unused_mut, unused_variables)]


fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        // 1. Handle the command-line arguments!
        //    If arg is "sum", then call the sum() function
        //    If arg is "double", then call the double() function
        //    If arg is anything else, then call the count() function
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }

    }
}

fn sum() {
    let mut sum = 0;
    // 2. Use a "for loop" to iterate through integers from 7 to 23 *inclusive*
    // and add them all together (find the sum).  Hint: You should get 255
    for i in 7..=23 {
        sum += i;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    // Use a "while loop" to count how many times you can double the value of x
    // (multiply x by 2) before it is larger than 500.  Hint: count should be 9
    while x <= 500 {
        x *= 2;
        count += 1;
    }

    println!("You can double x {} times before it is larger than 500", count);
}

fn count(arg: String) {
    // Use an unconditional loop to print arg 8 times, and then break.
    //
    let mut i = 0;
    loop {
        if i == 8 {
            break;
        }
        println!("{} ", arg); // Do this 8 times, and then break
        i += 1;
    }


    println!(); // This will output just a newline
}