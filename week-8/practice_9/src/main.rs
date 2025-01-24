fn main() {
    let b: (i32, bool, f64) = (110, true, 20.9);
    println!("{:?}", b);

    // Pass the tuple as a parameter
    print_tuple(b);
}

fn print_tuple(x: (i32, bool, f64)) {
    println!("Inside print method");
    println!("{:?}", x);
}
