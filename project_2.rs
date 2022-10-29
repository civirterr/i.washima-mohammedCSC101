fn main() {
	let t:f64 = 450_000_000.00 + 450_000_000;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00 + 750_000.00 + 750_000.00;
	let d:f64 = 2_850_000.00 + 2_850_000.00 + 2_850_000.00;
	let a:f64 = 250_000.00;
 
	let s:f32 = t + m + h + d + a;
	let dn:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;
	let avg = s/dn;

	println!("Sum is {}", s);
	println!("Average is{}", avg);


}