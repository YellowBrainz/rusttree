//use hex_literal::hex;
//use sha3::{Digest, Sha3_256};
use std::env;

const HASH_FUNCTION: &str = "NEWLEGACYKECCAK256";

#[derive(Debug)]
struct Tree {
	depth: u32,
	hash_type: String,
	version: String
}

impl Tree {
	fn new(d: u32, v: &str) -> Tree {
		Tree {
			depth: d,
			hash_type: HASH_FUNCTION.to_string(),
			version: v.to_string()
		}
	}
}

trait ValidateDepth {
	fn validate_depth(&self) -> bool;
}

// ValidateDepth to check if we have a legal tree
impl ValidateDepth for Tree {
	fn validate_depth(&self) -> bool {
		if self.depth > 11 || self.depth <= 1 {
			println!("[X] Depth should be > 1 and <= 11: {}", self.depth);
			false
		}
		else {
			true
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() == 3 || args.len() == 2 {
		match &args[args.len()-1].parse::<u32>() {
			Ok(n) => build_tree(*n),
			Err(e) => println!("[x] ERROR! Need depth value; {}", e),
		}
	}
	else {
		println!("[x] ERROR! Need depth argument");
		return;
	}

	return;
/*
	let mut size = 1000;

	size = size * 3000000000;
	let mut values: Vec<u64> = vec![0; size];

	for i in 0..2999 {
		values[i] = i as u64;
	}

	println!("Element 999: {} {}", values[999], usize::MAX);

	// create a SHA3-256 object
	let mut hasher = Sha3_256::new();

	// write input message
	hasher.update(b"abc");

	// read hash digest
	let result = hasher.finalize();

	println!("0x{:x}", result);
	println!("{:?}", hex!("3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"));
	assert_eq!(result[..], hex!("3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532")[..]);
*/
}

fn build_tree(depth: u32) {
	println!("[+] Building tree with depth = {}", depth);

	// Calculate the number of nodes in this tree
	let base: i32 = 2;
	let number_of_nodes: usize = base.pow(depth + 1) as usize;
	let number_of_leaves: u64  = base.pow(depth) as u64;

	println!("[-] number of nodes  : {}", number_of_nodes);

	let mut values: Vec<u64> = vec![0; number_of_nodes];

	println!("[-] number of leaves : {}", number_of_leaves);

	for i in 0..number_of_nodes {
		values[i] = i as u64;
	}

	let tree = Tree::new(depth, "20211219.0");
	if !tree.validate_depth() {
		return
	}

	println!("[.] ALL OK! {} {}", tree.hash_type.to_string(), tree.version.to_string());
}