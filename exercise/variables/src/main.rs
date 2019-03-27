fn main() {
    let mut missiles = 8;
    let loaded = 2;
    println!("Firing {} of my {} missiles...", loaded, missiles);
    missiles = missiles - loaded;
    println!("{} missiles left", missiles);
}
