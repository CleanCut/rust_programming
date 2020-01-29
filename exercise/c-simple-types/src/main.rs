// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

use ding_machine::{print_difference, print_array, ding, on_off};

fn main() {

    let coords: (f32, f32) = (6.3, 15.0);

    let coord_arr: [(f32, f32); 5] = [coords; 5];

    // 1. Use print_difference() to show the difference between the two numbers
    // in coords.  Use tuple indexing. Hint: It's NOT square brackets.
    //
    print_difference(coords.0, coords.1);


    // 2. We want to use print_array() to print coords...but coords isn't an array!
    //    Create an array of type [f32; 2] and initialize it to contain the
    //    information from coords.  Uncomment the print_array line and run the code.
    //
    let coords_arr: [f32; 2] = [coords.0, coords.1];               // create the array here
    print_array(coords_arr);


    let series = [1, 1, 2, 3, 5, 8, 13];
    // 3. Make the ding() function happy by passing it the value 13 out of the
    // series array.  Use array indexing.
    //
    ding(series[6]);


    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // 4. Pass the on_off function the value "true" from mess.  I'll get you
    //    started:
    //
    on_off(mess.2[1].0);

    // 5.  What a mess -- functions in a binary! Let's get organized!
    //
    //   - Make a library file
    //   - Move all the functions (except main) into the library
    //   - Bring all the functions into scope using use statements

    // Challenge: Uncomment the line below, run the code, and examine the
    // output. Then go refactor the print_distance() function according to the
    // instructions in the comments inside that function.

    // print_distance(coords);
}

