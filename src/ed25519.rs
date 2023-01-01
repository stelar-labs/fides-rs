use std::error::Error;

use ed25519_dalek::{ Keypair, PublicKey, Verifier, SecretKey, Signature, Signer };
use rand::{ RngCore, rngs::OsRng };

pub fn secret_key() -> [u8; 32] {

    let mut secret_key: [u8; 32] = [0_u8; 32];
    
    let mut key = [0_u8; 32];

    OsRng.fill_bytes(&mut key);

    while secret_key == [0_u8; 32] {
    
        match SecretKey::from_bytes(&key) {

            Ok(r) => secret_key = r.to_bytes(),

            Err(_) => OsRng.fill_bytes(&mut key)

        }

    }

    secret_key

}

pub fn public_key(secret_key: &[u8; 32]) -> Result<[u8; 32], Box<dyn Error>> {

    let sk: SecretKey = SecretKey::from_bytes(&*secret_key)?;

    let pk: PublicKey = PublicKey::from(&sk);

    Ok(pk.to_bytes())

}

pub fn sign(
    message: &[u8],
    secret_key: &[u8; 32]
) -> Result<[u8; 64], Box<dyn Error>> {

    let pubic_key = public_key(secret_key)?;

    let key_pair_bytes: Vec<u8> = [secret_key.to_vec(), pubic_key.to_vec()].concat();

    let key_pair: Keypair = Keypair::from_bytes(&key_pair_bytes)?;

    let signature: Signature = key_pair.sign(message);

    Ok(signature.to_bytes())

}

pub fn verify(
    message: &[u8],
    public_key: &[u8; 32],
    signature: &[u8; 64]
) -> Result<bool, Box<dyn Error>> {

    let pk: PublicKey = PublicKey::from_bytes(public_key)?;

    let sg: Signature = Signature::from_bytes(signature)?;

    match pk.verify(message, &sg) {

        Ok(_) => Ok(true),
        
        Err(_) => Ok(false),
    
    }

}
