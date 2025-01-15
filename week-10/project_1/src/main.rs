struct Purchase {
    hp:i32,
    ibm:i32,
    toshiba:i32,
    dell:i32,
}
impl Purchase {
    fn cost(&self)->i32{
     (3*self.hp)+(3*self.ibm)+(3*self.toshiba)+(3*self.dell)
    }
}
fn main() {
    let a = Purchase{
        hp:650000,
        ibm:755000,
        toshiba:550000,
        dell:850000
    };

    println!("1 hp is {} \n 1 ibm is {} \n 1 toshiba is {} \n 1 dell is {} \n 
        Total cost is {}",a.hp,a.ibm,a.toshiba,a.dell,a.cost());
}
