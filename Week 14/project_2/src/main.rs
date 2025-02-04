use std::fs::File;
use std::io::Read;

fn main() {
    // Open the SQL file
    let mut file = File::open("staff_tb.sql")
        .expect("Failed to open SQL file");
    
    // Create a buffer to store contents
    let mut contents = String::new();
    
    // Read file contents into string
    file.read_to_string(&mut contents)
        .expect("Failed to read SQL file");
    
    // Print the SQL contents
    println!("SQL File Contents:\n{}", contents);
}
