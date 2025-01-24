fn main() {
    // Using Vec::new()
    let v: Vec<i64> = Vec::new();
    println!("The length of Vector is: {}", v.len());

    // Using macro
    let v = vec!["Grace", "Effiong", "Basil", "Careen", "Susan"];
    println!("The length of vec macro is: {}", v.len());
}
