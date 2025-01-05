use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Names of commissioners
    let name = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", 
                    "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];

    // Ministry vector
    let ministry = vec!["Internal Affairs", "Justice", "Defense",
                         "Power & Steel", "Petroleum"];

    // Geopolitical zone vector
    let geo_zone = vec!["South West", "North East", "South South", "South West", 
                        "South East"];

    // Open or create the file to write to
    let mut file = File::create("commissioners_info.txt")?;

    // Write welcome message
    writeln!(file, "\nWelcome to the Information Service Department of Nigeria (Abuja)\n")?;
    writeln!(file, "\nCommissioners Information:\n")?;

    // Write the information of each commissioner to the file
    for i in 0..name.len() {
        writeln!(
            file,
            "Name: {}\nMinistry: {}\nGeopolitical Zone: {}\n\n", 
            name[i], ministry[i], geo_zone[i]
        )?;
    }

    println!("Commissioners' information has been written to commissioners_info.txt");

    Ok(())
}
