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
    print!("Input a for Administrator");
    print!("Input p for Projectmanager");
    print!("Input c for Customer");
    print!("Input d for Dataplan");
    print!("Please Input your level");

    let userinput = userinput.trim();
    io::stdin().read_line(&mut userinput).expect("Failed to read input");

    if userinput =="e" {
        let r = employee();
    } else if userinput =="a" {
        let r = administrator();
    }else if userinput =="p"{
        let r = projectmanager();
    }else if userinput =="c" {
        let r = customer();
    }else if userinput =="d" {
        let r = dataplan();
    }
}
