#[allow(dead_code)]

mod hash {

	use sha2::Sha256;
	use sha2::Digest;

	pub fn factorial(num: u128) -> u128 {
		match num {
			0 => 1,
			1 => 1,
			_ => factorial(num - 1) * num,
		}
	}

	pub fn hash_count(alphabet_length: usize, length: usize) -> u128 {
		alphabet_length.pow(length as u32) as u128
	}

	pub fn total_hash_count(alphabet_length: usize, min_length: usize, max_length: usize) -> u128 {
		(min_length..=max_length).fold(0 as u128, |acc, x| acc + hash_count(alphabet_length, x))
	}

	pub fn relative_index_to_absolute(relative_index: u128, alphabet_length: usize, min_length: usize, _max_length: usize) -> u128 {
		(1..min_length).map(|x| alphabet_length.pow(x as u32) as u128).sum::<u128>() + relative_index
	}

	pub fn absolute_index_to_relative(absolute_index: u128, alphabet_length: usize, _at_length: usize) -> u128 {
		let mut length: usize = 1;
		let mut curr_length: u128 = alphabet_length as u128;
		loop {
			if curr_length >= absolute_index {
				break;
			}
			length += 1;
			curr_length += hash_count(alphabet_length, length);
		}
		absolute_index - curr_length

	}

	pub fn hash(_alphabet_size: usize, _absolute_index: u128, _min_length: usize, _max_size: usize) -> () {
		let mut hasher = Sha256::new();
		hasher.update("");
		hasher.finalize();
	}

	pub fn hash_to_relative(hash: &String, hash_count: u128) -> u128 {
		let nb_bits = (hash_count as f64).sqrt().ceil() as usize;
		let h = &hash[..nb_bits];
		let mut i = u128::from_str_radix(&h, 16).unwrap();
		if i >= hash_count {
			i %= hash_count;
		}
		return i;
	}

	pub fn relative_index_to_text(relative_index: u128, alphabet_length: usize, min_length: usize, max_length: usize) -> String {
		let hash_count = total_hash_count(alphabet_length, min_length, max_length);
		if relative_index > hash_count {
			panic!("relative index greater than hash count");
		}
		return String::from("");
	}
}
