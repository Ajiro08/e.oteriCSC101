use std::fs::File;
use std::io::Write;
use std::io;

fn main() -> io::Result<()> {
    let mut file = File::create("products.txt")?;
    
    let data = "
    Nigerian Breweries Products:
    
    Lager:
    - 33 Export
    - Desperados
    - Goldberg
    - Gulder
    - Heineken
    - Star

    Stout:
    - Legend
    - Turbo King
    - Williams

    Non-Alcoholic:
    - Maltina
    - Amstel Malta
    - Malta Gold
    - Fayrouz
    ";

    file.write_all(data.as_bytes())?;
    println!("Data successfully written to products.txt");

    Ok(())
}
