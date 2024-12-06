fn main() {
    //Using Vec::new()
    let v: Vec<f64> =Vec::new();

    //printing the size of vector
    println!("\n The length of Vec::new is: {}", v.len());

    //Using macro
    let v = vec!["Grace","Effiong","Basil","Kareen","Susan"];

    //printing the size of vector
    println!("\n The length of vec macro is: {}",v.len());
}
