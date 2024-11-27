fn main () {
	let t:f64 = 2.0*(450000.00);
	let m:f64 = 1.0*(1500000.00);
	let h:f64 = 3.0*(750000.00);
	let d:f64 = 3.0*(2850000.00);
	let a:f64 = 1.0*(250000.00);

	let q = 2.0+1.0+3.0+3.0+1.0;
	println!("quantity is {}",q ); 

	let s = t+m+h+d+a;
	println!("sum is {}",s );

	let a = s/q;
	println!("average is {}",a );

}