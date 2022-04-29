pub mod chacha20poly1305;
pub mod ed25519;
pub mod x25519;
use blake3;

pub fn hash(input: &[u8]) -> [u8;32] {
    * blake3::hash(input).as_bytes()
}

pub fn merkle_root(mut hashes: Vec<[u8; 32]>) -> [u8; 32] {

	let mut result = [0_u8; 32];

	if !hashes.is_empty() {

		if hashes.len() % 2 != 0 {
            
            hashes.push([0_u8; 32])
        
        };

		while hashes.len() > 1 {

			let mut cache: Vec<[u8; 32]> = Vec::new();

			let mut inter: Vec<[u8; 32]> = Vec::new();

			for h in hashes {
				
				inter.push(h);
				
				if inter.len() == 2 {

					let joined_hashes = [inter[0], inter[1]].concat();
						
                    cache.push(hash(&joined_hashes[..]));

                    inter.clear()

				}

			}

			hashes = cache

		};

		result = hashes[0]

	}

	result

}
