fn main() {
    // Sales records: (Item, Quantity, Amount)
    let sales_records = vec![
        ("Toshiba", 2, 450_000.00),
        ("Mac", 1, 1_500_000.00),
        ("HP", 3, 750_000.00),
        ("Dell", 3, 2_850_000.00),
        ("Acer", 1, 250_000.00),
    ];

    let (total_amount, average_amount) = calculate_sum_and_average(&sales_records);

    println!("Total Amount: {:.2}", total_amount);
    println!("Average Amount: {:.2}", average_amount);
}
