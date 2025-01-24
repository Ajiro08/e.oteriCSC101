use std::collections::HashMap;

fn main() {
    // Define a vector of tuples for staff information
    let staff_levels: Vec<(&str, &str, &str, &str)> = vec![
        ("Intern", "Office Administrator", "APS 1-2", "Paralegal"),
        ("Administrator", "Academic", "APS 3-6", "Junior Associate"),
        ("Senior Administrator", "Lawyer", "APS 5-8", "Classroom Teacher"),
        ("Senior Associate 1", "PhD Candidate", "EL1 8-10", "Snr Teacher"),
        ("Director", "Post-Doc Researcher", "EL 2 10-13", "Leading Teacher"),
        ("CEO", "Senior Lecturer", "SES", "Deputy Principal"),
    ];

    // Create a HashMap to map roles to their respective APS levels
    let mut role_map: HashMap<&str, &str> = HashMap::new();
    for (role, _, aps, _) in &staff_levels {
        role_map.insert(role, aps);
    }

    // Example staff member to validate
    let staff_role = "Senior Associate 1";
    let years_of_experience = 7; // Example experience

    // Validate staff level
    match role_map.get(staff_role) {
        Some(&aps_level) => {
            if validate_aps_level(aps_level, years_of_experience) {
                println!("{} holds position: {}", staff_role, aps_level);
            } else {
                println!("{} does not hold a valid position for {} years of experience.", staff_role, years_of_experience);
            }
        }
        None => println!("Role not found: {}", staff_role),
    }
}

// Function to validate the APS level based on years of experience
fn validate_aps_level(aps: &str, years: i32) -> bool {
    match aps {
        "APS 1-2" => years <= 2,
        "APS 3-6" => years >= 3 && years <= 6,
        "APS 5-8" => years >= 5 && years <= 8,
        "EL1 8-10" => years >= 8 && years <= 10,
        "EL 2 10-13" => years >= 10 && years <= 13,
        "SES" => years >= 15,
        _ => false,
    }
}
