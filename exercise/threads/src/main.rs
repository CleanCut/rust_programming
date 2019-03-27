use std::thread::spawn;

fn main() {
    let msg = "Hello!".to_string();
    let join_handle = spawn(move || {
        println!("{}", msg);
    });
    println!("Goodbye!");
    join_handle.join();
}
