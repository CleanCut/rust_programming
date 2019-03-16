# Answers A: Multiply

### Part 1
- `cargo new variables`

```rust
// main.rs

fn main() {
    let missiles = 8;
    let loaded = 2;
    println!("Firing {} of my {} missiles...", loaded, missiles);
}

```

### Part 2

```rust
fn main() {
    let mut missiles = 8;
    let loaded = 2;
    println!("Firing {} of my {} missiles...", loaded, missiles);
    missiles = missiles - loaded;
    println!("{} missiles left", missiles);
}
```

### Extra challenges

- Explicitly annotate the variables with the type `i32`

```rust
fn main() {
    let mut missiles: i32 = 8;
    let loaded: i32 = 2;
    println!("Firing {} of my {} missiles...", loaded, missiles);
    missiles = missiles - loaded;
    println!("{} missiles left", missiles);
}
```

- Try binding the variables all at once on one line using a pattern (parenthesis and commas) -- can you figure out where "mut" goes?

```rust
fn main() {
    let (mut missiles, loaded) = (8, 2);
    println!("Firing {} of my {} missiles...", loaded, missiles);
    missiles = missiles - loaded;
    println!("{} missiles left", missiles);
}
```

- [ ] Can you figure out the correct type annotation when you assign them all in one line? Hint: it will use the same sort of pattern.

```rust
fn main() {
    let (mut missiles, loaded): (i32, i32) = (8, 2);
    println!("Firing {} of my {} missiles...", loaded, missiles);
    missiles = missiles - loaded;
    println!("{} missiles left", missiles);
}

```

- Instead of changing missiles, just print `missiles - loaded` in the second `println(...)`
  - What does cargo say when you run your program?  

It gives this warning:

```
warning: variable does not need to be mutable
```


It gives this warning:

```
warning: unused variable: `jet`
```
