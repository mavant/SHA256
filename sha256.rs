use std::str;
use std::num;

fn rotr(num: u32, count: u32) -> u32 {
	if count == 0 {
		num
	} else {
		(num << count) | (num >> (32 - count))
	}
}

fn main() {
	let mut h0:u32 = 0x6a09e667;
	let mut h1:u32 = 0xbb67ae85;
	let mut h2:u32 = 0x3c6ef372;
	let mut h3:u32 = 0xa54ff53a;
	let mut h4:u32 = 0x510e527f;
	let mut h5:u32 = 0x9b05688c;
	let mut h6:u32 = 0x1f83d9ab;
	let mut h7:u32 = 0x5be0cd19;

	let k =
		[0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
		0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
		0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
		0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
		0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
		0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
		0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
		0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2];

	let mut msg: ~[u8] = "foo".as_bytes().to_owned();
	let l = msg.len() as u64;
	msg.push(0b10000000u8);

	while (msg.len() % 64) !=  56 {
		msg.push(0);
	}
	std::io::extensions::u64_to_be_bytes(l, 8, |v| for i in v.iter() { msg.push(*i);});

	for c in msg.mut_chunks(64) {
		let mut w: [u32,..64] = [0, ..64];
		for i in range(0, 16) {
			let a1: u32 = (c[4 * i] as u32) * (num::pow(2, 24) as u32);
			let a2: u32 = (c[4 * i + 1] as u32) * (num::pow(2, 16) as u32);
			let a3: u32 = (c[4 * i + 2] as u32) * (num::pow(2, 8) as u32);
			let a4: u32 = (c[4 * i + 3] as u32);
			let a: u32 = a1 + a2 + a3 +a4;
			w[i] = a;
		}

		for i in range(16, 64) {
			let s0 = rotr(w[i-15], 7) ^ rotr(w[i-15], 18) ^ rotr(w[i-15], 3);
      let s1 = rotr(w[i-2], 17) ^ rotr(w[i-2], 19) ^ rotr(w[i-2], 10);
      w[i] = w[i-16] + s0 + w[i-7] + s1;
		}

		let mut a = h0;
		let mut b = h1;
		let mut c = h2;
		let mut d = h3;
		let mut e = h4;
		let mut f = h5;
		let mut g = h6;
		let mut h = h7;

		for i in range(0, 64) {
			let S1 = rotr(e, 6) ^ rotr(e, 11) ^ rotr(e, 25);
	    let ch = (e & f) ^ (!e & g);
	    let temp1 = h + S1 + ch + k[i] + w[i];
	    let S0 = rotr(a, 2) ^ rotr(a, 13) ^ rotr(a, 22);
	    let maj = (a & b) ^ (a & c) ^ (b & c);
	    let temp2 = S0 + maj;

		  h = g;
	    g = f;
	    f = e;
	    e = d + temp1;
	    d = c;
	    c = b;
	    b = a;
	    a = temp1 + temp2;
		}

		h0 += a;
		h1 += b;
		h2 += c;
		h3 += d;
		h4 += e;
		h5 += f;
		h6 += g;
		h7 += h;
	}

	println!("{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}", h0, h1, h2, h3, h4, h5, h6, h7);
}
