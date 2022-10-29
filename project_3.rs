fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 =3.0;

	//Amount
	let d = 1.0 - (r/100.0);
	let d = f64::powf(d,n);

	let dep = p *d;
	println!("Depreciation is {}", dep);
}