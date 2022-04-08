pub mod ed25519;
pub mod x25519;
pub mod chacha20poly1305;

use blake3;

pub fn hash(input: &Vec<u8>) -> [u8;32] {
    * blake3::hash(input).as_bytes()
}

pub fn merkle_root(objects: Vec<Vec<u8>>) -> [u8; 32] {

    let mut hashes: Vec<[u8; 32]> = objects
        .iter()
        .map(|x| hash(x))
        .collect();

	if hashes.is_empty() {

		[0_u8; 32]

	} else {

		if hashes.len() % 2 != 0 {
            
            hashes.push([0_u8; 32])
        
        };

		while hashes.len() > 1 {

			let mut cache: Vec<[u8; 32]> = Vec::new();

			let mut inter: Vec<[u8; 32]> = Vec::new();

			for h in &hashes {
				
				inter.push(*h);
				
				if inter.len() == 2 {
						
                    cache.push(hash(&[inter[0].to_vec(), inter[1].to_vec()].concat()));

                    inter.clear()

				}
			}
			hashes = cache
		};
		hashes[0]
	}
}
