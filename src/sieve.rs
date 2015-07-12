
pub fn find_primes(upto: u32) -> Vec<u32> {
	let mut primes = vec![0;0];
	let mut numbers = &mut vec![0; upto as usize];

	let max = (upto as f64).sqrt() as u32;
	for n in 2..upto {
		if numbers[n as usize] == 1 {
			continue;
		}

		primes.push(n);
		let mut i = n;
		loop {	
			i += n;
			if (i >= upto){
				break;
			}

			numbers[i as usize] = 1;
		}
	}
	primes
}