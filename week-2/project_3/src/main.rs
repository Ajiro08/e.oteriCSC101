fn main() {
    let initial_value: f64 = 510_000.0;
    let depreciation_rate: f64 = 5.0;
    let years: i32 = 3;

    let final_value = calculate_depreciated_value(initial_value, depreciation_rate, years);

    println!("The value of the TV after {} years is: {:.2}", years, final_value);
}

fn calculate_depreciated_value(initial_value: f64, depreciation_rate: f64, years: i32) -> f64 {
    initial_value * (1.0 - (depreciation_rate / 100.0)).powf(years as f64)
}
