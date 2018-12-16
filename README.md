# Rust: A Crash Course

Just watching the training will be entertaining and informative, but you will truly learn a lot more if you actually
dig in and do some coding!  This repository is for you hands-on-learners who are ready to roll.

I use macOS, and that is what I developed this course on.  Everything _ought_ to work similarly on major Linux
distributions and Windows. Please do the following preparation _before_ the training so you can focus your time on
learning Rust.  Please [contact me](mailto:nathan.stocks@gmail.com) ASAP if you have trouble with anything on this page.


# Before the training...

## Install Rust

Rust 1.31.0 or newer is required for this course!

- Go to [rust-lang.org](https://rust-lang.org) and click on the `Get Started`
   button and follow the instructions to install Rust for your operating system.
   - Please DO NOT install rust via some other package manager, because it will be a version that is _too old_.

You should get somewhat similar output if you run commands like the ones below (newer versions are okay).  If you get a
version older than 1.31.0, then run `rustup update` to install a newer version.

```shell
$ rustc --version
rustc 1.31.0 (abe02cefd 2018-12-04)

$ cargo --version
rustc 1.31.0 (abe02cefd 2018-12-04)
```

- Clone or download this repository to the computer you will be using during the live training.

## Prepare to Learn

Please do the following (see the [How To Learn Rust](https://github.com/CleanCut/rust_a_crash_course/blob/master/HowToLearnRust.md)
page for details on all of these)
- [ ] Choose an IDE (or Editor) and configure it with Rust support and customize it to your liking
- [ ] Choose one place to "find answers" and either introduce yourself (if it's a forum, IRC, etc.) or find the answer
      to one question you have.
- [ ] Try doing something in Rust!  If you don't have a better idea, then just do this:
  - `cargo new message`
  - `cd message`
  - `cargo run`
  - Edit `src/main.rs` and change the message.
  - `cargo run` again to see your new message.
- [ ] Check out the descriptions of the tools and books.

# Training!

Now you are ready for the training!  

### Resources

- Live training by the instructor (Nathan Stocks)
- This Repository
- [How To Learn Rust](https://github.com/CleanCut/rust_a_crash_course/blob/master/HowToLearnRust.md)
- [The Rust Standard Library](https://doc.rust-lang.org/std/)
