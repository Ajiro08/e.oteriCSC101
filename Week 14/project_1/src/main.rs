use std::collections::HashMap;

// Define the database schema and tables
struct DatabaseSchema {
    users: HashMap<String, String>,
    projects: HashMap<String, Vec<String>>,
    staff: HashMap<String, Vec<String>>,
    customers: HashMap<String, Vec<String>>,
    dataplans: HashMap<String, Vec<String>>,
}

impl DatabaseSchema {
    fn new() -> Self {
        DatabaseSchema {
            users: HashMap::new(),
            projects: HashMap::new(),
            staff: HashMap::new(),
            customers: HashMap::new(),
            dataplans: HashMap::new(),
        }
    }

    fn get_structure(&self, user_role: &str) -> Vec<String> {
        match user_role {
            "administrator" => self.get_database_structure(),
            "project_manager" => self.get_project_structure(),
            "employee" => self.get_staff_structure(),
            "customer" => self.get_customer_structure(),
            "vendor" => self.get_dataplan_structure(),
            _ => vec!["Invalid user role.".to_string()],
        }
    }

    fn get_database_structure(&self) -> Vec<String> {
        let mut structure = Vec::new();
        structure.push("Database schema:".to_string());
        for (table_name, _) in self.users.iter() {
            structure.push(format!("- {}", table_name));
        }
        for (table_name, _) in self.projects.iter() {
            structure.push(format!("- {}", table_name));
        }
        for (table_name, _) in self.staff.iter() {
            structure.push(format!("- {}", table_name));
        }
        for (table_name, _) in self.customers.iter() {
            structure.push(format!("- {}", table_name));
        }
        for (table_name, _) in self.dataplans.iter() {
            structure.push(format!("- {}", table_name));
        }
        structure
    }

    fn get_project_structure(&self) -> Vec<String> {
        let mut structure = Vec::new();
        structure.push("Project table structure:".to_string());
        if let Some(columns) = self.projects.get("project_table") {
            for column in columns {
                structure.push(format!("- {}", column));
            }
        }
        structure
    }

    fn get_staff_structure(&self) -> Vec<String> {
        let mut structure = Vec::new();
        structure.push("Staff table structure:".to_string());
        if let Some(columns) = self.staff.get("staff_table") {
            for column in columns {
                structure.push(format!("- {}", column));
            }
        }
        structure
    }

    fn get_customer_structure(&self) -> Vec<String> {
        let mut structure = Vec::new();
        structure.push("Customer table structure:".to_string());
        if let Some(columns) = self.customers.get("customer_table") {
            for column in columns {
                structure.push(format!("- {}", column));
            }
        }
        structure
    }

    fn get_dataplan_structure(&self) -> Vec<String> {
        let mut structure = Vec::new();
        structure.push("Dataplan table structure:".to_string());
        if let Some(columns) = self.dataplans.get("dataplan_table") {
            for column in columns {
                structure.push(format!("- {}", column));
            }
        }
        structure
    }
}

fn main() {
    let mut database = DatabaseSchema::new();

    // Populate the database schema and tables
    database.users.insert("admin".to_string(), "password".to_string());
    database.projects.insert("project_table".to_string(), vec!["project_id", "project_name", "project_description"].iter().map(|s| s.to_string()).collect());
    database.staff.insert("staff_table".to_string(), vec!["staff_id", "staff_name", "staff_department"].iter().map(|s| s.to_string()).collect());
    database.customers.insert("customer_table".to_string(), vec!["customer_id", "customer_name", "customer_address"].iter().map(|s| s.to_string()).collect());
    database.dataplans.insert("dataplan_table".to_string(), vec!["dataplan_id", "dataplan_size", "dataplan_duration", "dataplan_price"].iter().map(|s| s.to_string()).collect());

    // Get the database structure for different user roles
    let admin_structure = database.get_structure("administrator");
    let project_manager_structure = database.get_structure("project_manager");
    let employee_structure = database.get_structure("employee");
    let customer_structure = database.get_structure("customer");
    let vendor_structure = database.get_structure("vendor");

    println!("Administrator structure:\n{}", admin_structure.join("\n"));
    println!("Project manager structure:\n{}", project_manager_structure.join("\n"));
    println!("Employee structure:\n{}", employee_structure.join("\n"));
    println!("Customer structure:\n{}", customer_structure.join("\n"));
    println!("Vendor structure:\n{}", vendor_structure.join("\n"));
}
