use std::collections::HashMap;

fn main() {
    // Define the candidate information
    let candidates = vec![
        ("Candidate A", 5),
        ("Candidate B", 8),
        ("Candidate C", 3),
        ("Candidate D", 10),
        ("Candidate E", 7),
    ];

    // Create a HashMap to store the candidates and their experience
    let mut experience_map: HashMap<&str, i32> = HashMap::new();
    for (name, years) in &candidates {
        experience_map.insert(name, *years);
    }

    // Find the candidate with the highest years of experience
    let (highest_candidate, highest_experience) = experience_map.into_iter().max_by_key(|&(_, years)| years).unwrap();

    println!("The candidate with the highest years of programming experience is: {}", highest_candidate);
    println!("Years of experience: {}", highest_experience);
}
