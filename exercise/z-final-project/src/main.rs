// FINAL PROJECT
//
// Create an image processing application.  What it does and how it does it is
// up to you.  Look for comments labeled **OPTION** below.
//
// You should be able to complete at least 2 or 3 **OPTION** sections during the
// time provided for this course.  Feel free to keep going and complete them
// all, if you like!  Have fun and play around with the code.
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    //
    //
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            blur(infile, outfile);
        },

        // **OPTION**
        "brighten" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            brighten(infile, outfile);
        },
        // **OPTION**
        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x: u32 = args.remove(0).parse().expect("Failed to parse a number");
            let y: u32 = args.remove(0).parse().expect("Failed to parse a number");
            let width: u32 = args.remove(0).parse().expect("Failed to parse a number");
            let height: u32 = args.remove(0).parse().expect("Failed to parse a number");
            crop(infile, outfile, x, y, width, height);
        }
        // **OPTION**
        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let amount = args.remove(0);
            rotate(infile, outfile, amount);
        }
        // **OPTION**
        // Invert -- see the invert() function below

        // **OPTION**
        // Grayscale -- see the grayscale() function below

        // **OPTION**
        "generate" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            generate(outfile);
        }
        // ANOTHER EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        },
    }
}

// NOTE 1: Image processing is very CPU-intensive.  Your program will run
// *noticeably* faster if you run it with the --release flag.
//
//   cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//   cargo run --release blur image.png blurred.png


// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
// let positive_number: u32 = some_string.parse().expect("Failed to parse a number");


fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    println!("brighten INFILE OUTFILE");
    println!("crop INFILE OUTFILE x y width height");
    println!("rotate INFILE OUTFILE AMOUNT");
    println!("generate OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(2.0);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    let img2 = img.brighten(100);

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.

    img2.save(outfile).expect("Failed to write OUTFILE");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    let img2 = img.crop(x, y, width, height);

    img2.save(outfile).expect("Failed to write OUTFILE");
}

fn rotate(infile: String, outfile: String, amount: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!
    let img2 = match amount.as_str() {
        "90" => img.rotate90(),
        "180" => img.rotate180(),
        "270" => img.rotate270(),
        _ => img.rotate90(),
    };

    img2.save(outfile).expect("We didn't save");
}


fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
}

fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .grayscale() takes no arguments and converts the image in-place, so
    // you will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
}

fn generate(outfile: String) {
    let width = 512;
    let height = 512;
    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([32_u8, 128u8, 255]);
    }

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!
    imgbuf.save(outfile).expect("Something went wrong");
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
