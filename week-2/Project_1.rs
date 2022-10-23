fn main() {
	let p:f64 = 520_000_000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	// compound interest
	let a = 1.0 + p * (r/100.0);
	let a = f64::powf(a,n);

	let ci = a - p;
	print!("Compound Interest is {}", ci);

}