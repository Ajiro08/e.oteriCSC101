fn main() {
    let mut fees = 25_000; // The `mut` keyword makes the variable mutable
    println!("Fees is: {}", fees);

    fees = 35_000; // Now, reassigning a new value is allowed
    println!("Fees changed to {}", fees);
}
