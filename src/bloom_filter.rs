use opis::{Bit, Integer};
use crate::{hash::{blake_3, sha_2, sha_3}, BloomFilter};

 impl BloomFilter {

    pub fn new(members: usize) -> Self {
        BloomFilter { bits: vec![Bit::Zero; ((11_f64 * members as f64) / 2_f64.ln()).round() as usize] }
    }

    pub fn insert(&mut self, object: &[u8]) {

        for hash_id in 0..11 {

            let hash_int =
            
                match hash_id {

                    0 => hash_int_from_bytes(&blake_3(object)),

                    1 => hash_int_from_bytes(&sha_2::sha_224(object)),

                    2 => hash_int_from_bytes(&sha_2::sha_256(object)),

                    3 => hash_int_from_bytes(&sha_2::sha_384(object)),

                    4 => hash_int_from_bytes(&sha_2::sha_512(object)),

                    5 => hash_int_from_bytes(&sha_2::sha_512_224(object)),

                    6 => hash_int_from_bytes(&sha_2::sha_512_256(object)),

                    7 => hash_int_from_bytes(&sha_3::sha_224(object)),

                    8 => hash_int_from_bytes(&sha_3::sha_256(object)),

                    9 => hash_int_from_bytes(&sha_3::sha_384(object)),

                    10 => hash_int_from_bytes(&sha_3::sha_512(object)),
            
                    _ => 0
                    
                };

            let i = hash_int as usize % self.bits.len();

            self.bits[i] = Bit::One;

        }

    }

    pub fn search(&self, object: &[u8])  -> bool {
        
        let bits: Vec<Bit> = (0..11)
            .into_iter()
            .map(|x| {

                let hash_int =
            
                    match x {

                        0 => hash_int_from_bytes(&blake_3(object)),

                        1 => hash_int_from_bytes(&sha_2::sha_224(object)),

                        2 => hash_int_from_bytes(&sha_2::sha_256(object)),

                        3 => hash_int_from_bytes(&sha_2::sha_384(object)),

                        4 => hash_int_from_bytes(&sha_2::sha_512(object)),

                        5 => hash_int_from_bytes(&sha_2::sha_512_224(object)),

                        6 => hash_int_from_bytes(&sha_2::sha_512_256(object)),

                        7 => hash_int_from_bytes(&sha_3::sha_224(object)),

                        8 => hash_int_from_bytes(&sha_3::sha_256(object)),

                        9 => hash_int_from_bytes(&sha_3::sha_384(object)),

                        10 => hash_int_from_bytes(&sha_3::sha_512(object)),
                
                        _ => 0
                        
                    };

                let i = hash_int as usize % self.bits.len();

                self.bits[i]

            })
            .collect();

        bits.iter().all(|&x| x == Bit::One)

    }

    pub fn bits(&self) -> Vec<Bit> {
        self.bits.clone()
    }
 }

 impl From<&[u8]> for BloomFilter {
    fn from(bytes: &[u8]) -> Self {
        BloomFilter { bits: Integer::from(bytes).bits() }
    }

}

impl Into<Vec<u8>> for BloomFilter {
    fn into(self) -> Vec<u8> {
        Integer::from(&self.bits[..]).into()
    }
}

impl Into<Vec<u8>> for &BloomFilter {
    fn into(self) -> Vec<u8> {
        Integer::from(&self.bits[..]).into()
    }
}

fn hash_int_from_bytes(bytes: &[u8]) -> u64 {

    let mut int_byte = [0_u8; 8];

    let mut i = 0;

    for byte in bytes {

        int_byte[i] = int_byte[i] ^ byte;

        if i == 7 {
            
            i = 0
        
        } else {
            
            i += 1
        
        }

    }

    u64::from_le_bytes(int_byte)

}

#[cfg(test)]
mod tests {
    use crate::BloomFilter;

    #[test]
    fn search() {
        
        let objs = vec![
            vec![1], vec![2], vec![3]
        ];

        let mut bloom = BloomFilter::new(3);

        for obj in objs {

            bloom.insert(&obj);

        }

        assert_eq!(bloom.search(&vec![1]), true);
    }
}