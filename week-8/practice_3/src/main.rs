// Method to print the get value
fn main() {
    let v = vec!["R", "V", "S", "T", "A", "C", "I", "A", "N"];
    let mut input = String::new();
    
    println!("Enter an index value btw (0 - 8):");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // index is the non-negative value which is smaller than the size of the vector
    let index: usize = input.trim().parse().expect("Invalid input");
    
    // getting value at given index value
    let ch: Option<&str> = v.get(index);
    
    match ch {
        Some(value) => println!("Element of vector at index {}: {}", index, value),
        None => println!("No element found at index {}", index),
    }
}
