fn main() {
    // Initialize a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);

    println!("Original tuple: {:?}", mountain_heights);

    // Change 3rd and 4th elements of a mutable tuple
    mountain_heights.2 = "Annapurna";
    mountain_heights.3 = 8091;

    println!("Changed tuple: {:?}", mountain_heights);
}
