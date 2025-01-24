fn main() {
    let b: (i32, bool, f64) = (30, true, 4.9);
    println!("{:?}", b);

    // Destructuring the tuple
    let (age, is_male, cgpa) = b;
    println!("Age is {}, isMale is {}, cgpa is {}", age, is_male, cgpa);
}
