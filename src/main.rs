use base64::{engine::general_purpose, Engine};
use digest::Digest;
use md5::Md5;
use pkcs8::DecodePrivateKey;
use rand::thread_rng;
use rsa::RsaPublicKey;
use rsa::{Pkcs1v15Sign, RsaPrivateKey};

fn main() -> anyhow::Result<()> {
    let der_bytes = std::fs::read("rsa_private_key.der")?;
    let key: RsaPrivateKey = RsaPrivateKey::from_pkcs8_der(&der_bytes)?;

    let data = b"OpenId=HASH13900000002";

    let mut hasher = Md5::new();
    hasher.update(data);
    let hashed_data = hasher.finalize();

    let mut rng = thread_rng();
    let signature = key.sign_with_rng(&mut rng, Pkcs1v15Sign::new::<Md5>(), &hashed_data)?;

    let signature_base64 = general_purpose::STANDARD.encode(&signature);
    println!("Sign Base64:\n{}", signature_base64);

    let pub_key = RsaPublicKey::from(&key);

    let padding = Pkcs1v15Sign::new::<md5::Md5>();
    match pub_key.verify(padding, &hashed_data, &signature) {
        Ok(_) => println!("signature verification succeeded"),
        Err(e) => println!("signature verification failed: {}", e),
    }

    Ok(())
}
