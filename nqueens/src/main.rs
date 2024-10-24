// License CC0
// From: https://github.com/attractivechaos/plb2/blob/ec1fa2e4d235e099917a4d221f45d71aac156d4c/src/rust/nqueen.rs

#![no_main]

use entrypoint::io;

entrypoint::entrypoint!(main);

pub fn nq_solve(n: usize) -> usize {
	let mut a: Vec<i32> = vec![-1; n];
	let mut l: Vec<i32> = vec![0; n];
	let mut c: Vec<i32> = vec![0; n];
	let mut r: Vec<i32> = vec![0; n];
	let mut m = 0;
	let y0 = (1<<n) - 1;
	let mut k = 0;
	loop {
		let y = (l[k] | c[k] | r[k]) & y0;
		if (y ^ y0) >> (a[k] + 1) != 0 {
			let mut i = a[k] + 1;
			while i < n as i32 && (y & (1<<i)) != 0 {
				i += 1;
			}
			if k < n - 1 {
				let z = 1<<i;
				a[k] = i;
				k += 1;
				l[k] = (l[k-1]|z)<<1;
				c[k] = c[k-1]|z;
				r[k] = (r[k-1]|z)>>1;
			} else {
				m += 1;
				if k == 0 {
					break;
				}
				k -= 1;
			}
		} else {
			a[k] = -1;
			if k == 0 {
				break;
			}
			k -= 1;
		}
	}
	return m;
}

fn main() {
    io::println("Nqueens: counting how many ways to place N queens on a NxN board so that no two queens attack each other");
	io::println("Choose N: ");
    let n = unsafe { io::getchar() as usize };
	println!("There are {} solutions", nq_solve(n));
}
