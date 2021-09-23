
mod signer;

mod verifier;

pub fn sign() {
    signer::run()
}

pub fn verify() {
    verifier::run()
}