fn main() {
    let mut fees = 25_000; // Use `mut` to make the variable mutable
    println!("Fees is: {}", fees);

    fees = 35_000; // Now this works
    println!("Fees changed to {}", fees);
}
