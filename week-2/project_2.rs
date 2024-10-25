fn main() {
	let t:f64 = 450_000.00;
	let tq:f64 = 2.0;
	let m:f64 = 1_500_000.00;
	let mq:f64 = 1.0;
	let h:f64 = 750_000.00;
	let hq:f64 = 3.0;
	let d:f64 = 2_850_000.00;
	let dq:f64 = 3.0;
	let a:f64 = 250_000.00;
	let aq:f64 = 1.0;


	//Calculating sum and average of Sales Record for P.M. Okeke and Sons Ltd.

	 let s = t*tq + m*mq + h*hq + d*dq + a*aq;
	 println!("Sum of P.M Okeke Sales is {}", s);
	 let av = s/ (tq + mq + hq + dq + aq);
	 println!("Average of P.M Okeke Sales is {}", av);

}