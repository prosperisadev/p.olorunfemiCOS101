fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//Calculating Depreciation of Ms. Akudo TV Set in 3 years
	let a = p * ( 1.0 - ( r / 100.0)).powf(n);
	println!("Amount is {}", a);
	let d = p - a;
	println!("Depeciation of Ms. Akudo TV Set in 3 years is {}", d);
}