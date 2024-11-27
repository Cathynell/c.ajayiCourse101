fn main () {
	let p= 510000;
	let r = 5;
	let n = 3;

	let a = p*(1-(r/100))^n;
	println!("Depreciation is {}", a );

}