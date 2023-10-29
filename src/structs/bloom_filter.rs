use std::error::Error;

use opis::{Bit, Integer};
use crate::hash;
use crate::structs::BloomFilter;

 impl BloomFilter {

    pub fn new(accuracy: f64, members: usize) -> Result<BloomFilter, Box<dyn Error>> {

        let n = members as f64;

        if accuracy > 1.0 {
            return Err("False Positive Rate higher than 1")?;
        }

        // number of bits required
        let n_ln_p = n * accuracy.ln();

        let ln_2 = 2_f64.ln();

        let ln_2_sqr = ln_2.powi(2);

        let r1 = n_ln_p / ln_2_sqr;

        let m = -1.0 * r1;

        // number of hash functions
        let k = (m / n) * ln_2;

        let bits = vec![
            Bit::Zero;
            m.round() as usize
        ];
        
        let hashes = if k > 5.0 {
            5 as u8
        } else {
            k as u8
        };
        
        Ok(BloomFilter { bits, hashes })

    }

    pub fn insert(&mut self, object: &[u8]) {

        (0..self.hashes)
            .into_iter()
            .for_each(|x| {
                match x {
                    0 => set_index(&mut self.bits, hash::farm_hash_64(object)),
                    1 => set_index(&mut self.bits, hash::metro_hash_64(object)),
                    2 => set_index(&mut self.bits, hash::pearson(object)),
                    3 => set_index(&mut self.bits, hash::spooky_hash_v1(object)),
                    4 => set_index(&mut self.bits, hash::spooky_hash_v2(object)),
                    _ => ()
                }
            })

    }

    pub fn search(&self, object: &[u8])  -> bool {
        
        (0..self.hashes)
            .into_iter()
            .all(|x| {
                
                let object_hash = match x {
                    0 => hash::farm_hash_64(object),
                    1 => hash::metro_hash_64(object),
                    2 => hash::pearson(object),
                    3 => hash::spooky_hash_v1(object),
                    4 => hash::spooky_hash_v2(object),
                    _ => return false,
                };
        
                let i = hash::city_hash_64(object_hash) as usize % self.bits.len();
                self.bits[i] == Bit::One

            })

    }

    pub fn bits(&self) -> Vec<Bit> {
        self.bits.clone()
    }

 }

 fn set_index(bits: &mut [Bit], object: u64) {

    let i = hash::city_hash_64(object) as usize % bits.len();

    bits[i] = Bit::One;

 }

 impl TryFrom<&[u8]> for BloomFilter {

    fn try_from(arg: &[u8]) -> Result<Self, Box<dyn Error>> {

        let decoded: Vec<&[u8]> = astro_format::decode(&arg)?;

        if decoded.len() == 3 || decoded[1].len() != 1 {

            let bits = Integer::from(decoded[2]);

            let bit_count_int = Integer::from(decoded[0]);

            let bit_count: usize = (&bit_count_int).into();

            let r = BloomFilter {
                bits: bits.to_ext_bits(bit_count),
                hashes: decoded[1][0]
            };

            Ok(r)

        } else {

            Err("Invalid Bloom Filter Format!")?

        }

    }

    type Error = Box<dyn Error>;

}

impl Into<Vec<u8>> for &BloomFilter {

    fn into(self) -> Vec<u8> {

        let bit_count = self.bits.len();

        let bit_count_int = Integer::from(&bit_count);

        let bit_count_bytes: Vec<u8> = bit_count_int.into();
        
        let bit_bytes: Vec<u8> = Integer::from(&self.bits[..]).into();

        let r = astro_format::encode(vec![
            &bit_count_bytes[..],
            &[self.hashes][..],
            &bit_bytes[..]
        ]).unwrap();

        r

    }

}

#[cfg(test)]
mod tests {
    use crate::structs::BloomFilter;

    #[test]
    fn search() {
        
        let objs = vec![
            vec![1_u8], vec![2], vec![3]
        ];

        let mut bloom = BloomFilter::new(0.01, 3).unwrap();

        for obj in objs {

            bloom.insert(&obj);

        }

        assert_eq!(bloom.search(&vec![1]), true);
    }
}