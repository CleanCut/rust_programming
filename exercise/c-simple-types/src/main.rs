#![allow(dead_code, unused_variables)]

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    // 1. Use print_difference() to show the difference between the two numbers
    // in coords.  Use tuple indexing. Hint: It's NOT square brackets.
    //
    // print_difference(...)


    // 2. We want to use print_array() to print coords...but coords isn't an array!
    //    Create an array of type [f32; 2] and initialize it to contain the
    //    information from coords.
    //
    // let coords_arr...
    // print_array(coords_arr);


    let series = [1, 1, 2, 3, 5, 8, 13];
    // 3. Make the ding() function happy by passing it the value 13 out of the
    // series array.  Use array indexing.
    //
    // ding(...)


    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // 4. Pass the on_off function the value "true" from mess.  I'll get you
    //    started:
    //
    // on_off(mess.2


    // Challenge: This works just fine.  Uncomment it and then follow the
    // instructions above the print_distance() function definition.
    //
    // print_distance(coords);
}

fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}

// Using z.0 and z.1 is not nearly as nice as using x and y.  Lucky for
// us, Rust supports destructuring function arguments.  Try replacing "z" in the
// parameter list with "(x, y)" and adjust the calculation in the function body
// accordingly.
fn print_distance(z: (f32, f32)) {
    println!(
        "Distance to the origin is {}",
        (z.0.powf(2.0) + z.1.powf(2.0)).sqrt());
}

