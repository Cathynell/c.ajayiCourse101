use std::fs::File;
use std::io::{self,Read};

fn employee() -> io::Result<()> {
   let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut employee_data= String::new();
    file.read_to_string(&mut employee_data).unwrap();
    print!("{}", employee_data);
    Ok(())
 
}

fn administrator() -> io::Result<()>{
   let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut administrator_data =String::new();
    file.read_to_string(&mut administrator_data).unwrap();
    print!("{}", administrator_data); 
    Ok(())
}

fn project() -> io::Result<()>{

    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut projectmanager =String::new();
    file.read_to_string(&mut projectmanager).unwrap();
    print!("{}", projectmanager);
    Ok(()) 
}

fn customer() ->io::Result<()>{
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut customer =String::new();
    file.read_to_string(&mut customer).unwrap();
    print!("{}", customer);
    Ok(())
}

fn dataplan() ->io::Result<()>{
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut vendor =String::new();
    file.read_to_string(&mut vendor).unwrap();
    print!("{}", vendor);
    Ok(())
} 

fn main() {
    print!("\n Welcome to Globacom Ltd Database\n");
    print!("Input e for Employee");
    print!("\nInput a for Administrator\n");
    print!("\nInput p for Projectmanager\n");
    print!("\nInput c for Customer\n");
    print!("\nInput d for Dataplan\n");
    print!("\nPlease Input your level\n");

    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput).expect("Failed to read input");
    let userinput = userinput.trim();

    if userinput == "e" {
        if let Err(e) = employee() {
            eprintln!("Error: {}", e);
        }
    } else if userinput == "a" {
        if let Err(e) = administrator() {
            eprintln!("Error: {}", e);
        }
    } else if userinput == "p" {
        if let Err(e) = project() {
            eprintln!("Error: {}", e);
        }
    } else if userinput == "c" {
        if let Err(e) = customer() {
            eprintln!("Error: {}", e);
        }
    } else if userinput == "d" {
        if let Err(e) = dataplan() {
            eprintln!("Error: {}", e);
        }
    } else {
        println!("Invalid input, please try again.");
    }
}
