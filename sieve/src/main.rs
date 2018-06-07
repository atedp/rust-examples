extern crate primal;

use primal::Sieve;

let sieve = Sieve::new(10000);
	let suspect = 5273;
	println!("{} is a prime: {}", suspect, sieve.is_prime(suspect));
	let not_a_prime = 1024;
	println!("{} is a prime: {}", not_a_prime, sieve.is_prime(not_a_prime));
	let n = 1000;
