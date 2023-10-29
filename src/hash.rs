mod city_hash_64;
mod city_hash_128;
mod farm_hash_32;
mod farm_hash_64;
mod farm_hash_128;
mod lookup3_oat;
mod lookup3_ycs;
mod metro_hash_64;
mod metro_hash_128;
mod murmur_hash3_64;
mod pearson;
mod spooky_hash_v1;
mod spooky_hash_v2;

pub use crate::hash::city_hash_64::city_hash_64;
pub use crate::hash::city_hash_128::city_hash_128;
pub use crate::hash::farm_hash_32::farm_hash_32;
pub use crate::hash::farm_hash_64::farm_hash_64;
pub use crate::hash::farm_hash_128::farm_hash_128;
pub use crate::hash::lookup3_oat::lookup3_oat;
pub use crate::hash::lookup3_ycs::lookup3_ycs;
pub use crate::hash::metro_hash_64::metro_hash_64;
pub use crate::hash::metro_hash_128::metro_hash_128;
pub use crate::hash::murmur_hash3_64::murmur_hash3_64;
pub use crate::hash::pearson::pearson;
pub use crate::hash::spooky_hash_v1::spooky_hash_v1;
pub use crate::hash::spooky_hash_v2::spooky_hash_v2;

use blake3;

pub fn blake_3(input: &[u8]) -> [u8; 32] {
    * blake3::hash(input).as_bytes()
}
