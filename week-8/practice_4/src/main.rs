// More vector
let names = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];
let ages = vec![16, 17, 19, 22, 20, 21, 18, 23];

println!("\nAge allocation:");

// Loop to iterate elements in vector
for i in 0..ages.len() {
    // Iterating through i on the vector
    println!("{} is {} years old", names[i], ages[i]);
}
