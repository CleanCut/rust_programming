fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;
    // 1. Try running this code and take a look at the errors.
    //
    // See if you can fix the problem. It's right around here, somewhere.
    {
        let area = area_of(width, height);
        println!("Area is {}", area);
    }

    // 2. The area is incorrect! Go fix the area_of() function below, then run
    //    the code again and make sure it worked.

    // 3. Uncomment the line below.  Make it work by creating a "volume"
    //    function that multiplies three arguments together and returns the
    //    result.
    //
    println!("Volume is {}", volume(width, height, depth));
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}

fn area_of(x: i32, y: i32) -> i32 {
    // Fix this function to correctly compute the area of a rectangle given
    // dimensions x and y.
    //
    // Bonus: Idiomatic rust doesn't use "return" at the end of a block, fix it!
    x * y
}
