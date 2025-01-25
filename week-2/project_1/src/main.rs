fn main() {
    let principal: f64 = 520_000_000.0;
    let rate: f64 = 10.0;
    let time: i32 = 5;

    let compound_interest = calculate_compound_interest(principal, rate, time);

    println!("The compound interest for 5 years at 10% per annum is: {:.2}", compound_interest);
}

fn calculate_compound_interest(principal: f64, rate: f64, time: i32) -> f64 {
    let amount = principal * (1.0 + rate / 100.0).powf(time as f64);
    let compound_interest = amount - principal;
    compound_interest
}
