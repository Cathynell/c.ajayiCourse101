fn main() {
    let fullname = "Pan-Atlantic University";
    println!();
    print!("Name: {}", fullname);
    println!();
    println!("Before trim");
    println!("length is {}",fullname.len());
    println!();
    println!("After trim");
    println!("length is {}",fullname.trim().len());
}