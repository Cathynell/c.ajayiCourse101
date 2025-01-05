use std::io::Write;
fn main() {
    let input1 = "33 Export,";
    let input2 = " Desperados,";
    let input3 = " Goldberg,";
    let input4 = " Gulder,";
    let input5 = " Heineken,";
    let input6 = " and Star";


    let info1 = "Legend,";
    let info2 = " Turbo king,";
    let info3 = " and Williams";

    let a = "Maltina,";
    let b = " Amstel Malta,";
    let c = " Malta Gold,";
    let d = " and Fayrouz";

    let mut file = std::fs::File::create("Nigerian Breweries Plc.txt").expect("create failed");
    file.write_all("Welcome to Nigerian Brewery Limited\n".as_bytes()).expect("Write failed");
    file.write_all("We have the following brands -: [LAGER, STOUT, NON-ALCOHOLIC]\n".as_bytes()).expect("Write failed");
    file.write_all("LAGER: \n".as_bytes()).expect("Write failed");
    file.write_all(input1.as_bytes()).expect("Write failed");
    file.write_all(input2.as_bytes()).expect("Write failed");
    file.write_all(input3.as_bytes()).expect("Write failed");
    file.write_all(input4.as_bytes()).expect("Write failed");
    file.write_all(input5.as_bytes()).expect("Write failed");
    file.write_all(input6.as_bytes()).expect("Write failed");


    file.write_all("\nSTOUT: \n".as_bytes()).expect("Write failed");
    file.write_all(info1.as_bytes()).expect("Write failed");
    file.write_all(info2.as_bytes()).expect("Write failed");
    file.write_all(info3.as_bytes()).expect("Write failed");


    file.write_all("\nNON-ALCOHOLIC: \n".as_bytes()).expect("Write failed");
    file.write_all(a.as_bytes()).expect("Write failed");
    file.write_all(b.as_bytes()).expect("Write failed");
    file.write_all(c.as_bytes()).expect("Write failed");
    file.write_all(d.as_bytes()).expect("Write failed");

    println!("\n Drink written to file.");
}