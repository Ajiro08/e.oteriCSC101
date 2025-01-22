use std::fs::File;
use std::io::Write;
use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() -> io::Result<()> {
    let students = vec![
        Student {
            name: String::from("Oluchi Morol"),
            matric_number: String::from("ACC210211111"),
            department: String::from("Accounting"),
            level: 300,
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO210222111"),
            department: String::from("Economics"),
            level: 300,
        },
        Student {
            name: String::from("Shaina Bolade"),
            matric_number: String::from("CSC210822828"),
            department: String::from("Computer"),
            level: 200,
        },
        Student {
            name: String::from("Adekule David"),
            matric_number: String::from("EEE210222222"),
            department: String::from("Electrical"),
            level: 200,
        },
        Student {
            name: String::from("Bianca Edmond"),
            matric_number: String::from("MEI210200001"),
            department: String::from("Mechanical"),
            level: 100,
        },
    ];

    let mut file = File::create("students.csv")?;

    file.write_all(b"Student Name,Matric Number,Department,Level\n")?;
    for student in &students {
        let line = format!("{},{},{},{}\n", 
            student.name, 
            student.matric_number, 
            student.department, 
            student.level);
        file.write_all(line.as_bytes())?;
    }

    println!("Data successfully written to students.csv");

    Ok(())
}
