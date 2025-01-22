fn main() {
    // Define the commissioners
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Aészez Redu",
        "Okorocha Calixtus Ogbona",
        "Adewale Jimoh Akambi",
        "Osazuwa Faith Efejue",
    ];

    // Define the ministries
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // Define the geopolitical zones
    let geopolitical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Print the merged data
    println!("S/N\tName of Commissioner\t\t\tMinistry\t\tGeopolitical Zone");
    println!("----------------------------------------------------------------------");
    for i in 0..commissioners.len() {
        println!(
            "{}\t{: <30}\t{: <20}\t{}",
            i + 1,
            commissioners[i],
            ministries[i],
            geopolitical_zones[i]
        );
    }
}
