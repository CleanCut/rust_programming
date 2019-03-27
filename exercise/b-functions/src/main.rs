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

    // 3. Uncomment the line below.  Make it work by creating a "volume"
    //    function that multiplies three arguments together and returns the
    //    result.
    //
    println!("Volume is {}", volume(width, height, depth));
}

fn volume(width: i32, height: i32, depth: i32) -> i32 {
    width * height * depth
}

fn area_of(x: i32, y: i32) -> i32 {
    // 2. Fix this function to compute the area given dimensions x and y
    //
    // Bonus: Idiomatic rust doesn't use "return" at the end of a block, fix it!
    x * y
}
