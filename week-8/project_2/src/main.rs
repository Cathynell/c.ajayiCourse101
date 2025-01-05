use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Student name vector
    let name = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold",
                    "Blanca Edemoh"];

    // Matric number vector
    let matric_no = vec!["ACC10211111", "ECO10110101", "CSC10328828",
                          "EEE11020202", "MEE10202001"];

    // Department vector
    let department = vec!["Accounting", "Economics", "Computer", "Electrical",
                          "Mechanical"];

    // Level vector
    let level = vec!["300", "100", "200", "200", "100"];

    // Open a file in write mode
    let mut file = File::create("student_info.txt")?;

    // Write the welcome message
    writeln!(file, "\n Welcome to Student Management Information System!(PAU-SMIS)\n")?;
    writeln!(file, "\n PAU-SMIS\n")?;

    // Write the header
    writeln!(file, "{:<20} {:<15} {:<15} {:<10}", "Student Name", "Matric No", "Department", "Level")?;
    writeln!(file, "{:-<20} {:-<15} {:-<15} {:-<10}", "", "", "", "")?;

    // Write student data
    for i in 0..name.len() {
        writeln!(
            file,
            "{:<20} {:<15} {:<15} {:<10}",
            name[i], matric_no[i], department[i], level[i]
        )?;
    }

    println!("Student information has been written to student_info.txt");

    Ok(())
}

