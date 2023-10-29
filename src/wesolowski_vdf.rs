use std::error::Error;

use opis::Integer;
use sha3;

pub fn generate(
   input: Integer,
   prime_number: &Integer,
   security: Integer,
) -> Result<(Integer, Integer), Box<dyn Error>> {

   let mut result = input;

   let mut count = security.clone();

   while count > Integer::zero() {
      
      result = (&(&result * &result) % prime_number)?;

      count -= Integer::one()

   }
   
   let exp = ((prime_number - Integer::one()) / (Integer::two() * security) + Integer::one())?;
   
   let mut h = result;

   count = exp;
   
   while count > Integer::zero() {
      
      h = ((h * result) % prime_number)?;

      count -= Integer::one()
   
   }
   
   let w = h;


   let challenge_input: Vec<u8> = concat!(prime_number.into(), input.into(), w.into());

   let c = sha3::sha_256(challenge_input);

   let x = (result.mod_pow(c.into(), prime_number) * input.mod_pow(&w, prime_number)) % prime_number;

   return (result, x);

   Ok((result, proof))

}

pub fn verify(
   input: &Integer,
   security: &Integer,
   output: &Integer,
   prime_number: &Integer,
   proof: &Integer,
) -> Result<bool, Box<dyn Error>> {

   // Compute h(x) = x^((p-1)/2k+1) mod p
   let exp = ((prime_number - Integer::one()) / (Integer::two() * security) + Integer::one())?;

   let mut h = v;

   let mut count = exp.clone();

   while count > Integer::zero() {

   }

   for _ in 0..exp {

       h = (h * v) % p;
   }
   let w = h;

   // Compute the challenge value c
   let challenge_input = format!("{}{}{}", p, r, w);
   let c = Sha3_256::digest(challenge_input.as_bytes());
   let c = u64::from_le_bytes(c[..].try_into().unwrap());

   // Verify the proof
   let lhs = modexp(v, c, p) * modexp(r, w, p) % p;
   let rhs = x % p;

   // return lhs == rhs;











   let two = Integer::two();
   
   let mut v = output.clone();

   let mut iter_count = iterations.clone();

   while iter_count > Integer::zero() {

      let bi = (input >> &iter_count) & Integer::one();

      let v_2 = (&(&v * &v) % prime_number)?;

      iter_count -= Integer::one();
      
      if bi == Integer::zero() {
         
         v = v_2;
      
      } else {
         
         v = (&(&v_2 * quadratic_residue) % prime_number)?;
         
         v = (&(&v * &output) % prime_number)?;
      
      }
   
   }
   
   let proof_squared = (&(&proof * &proof) % prime_number)?;
   
   let expected = (&(v + proof_squared * quadratic_residue) % prime_number)?;
   
   Ok(expected == output)

}
