use anyhow::{bail, Result};
use base64::{engine::general_purpose, Engine as _};
use pkcs8::der::Decode;
use pkcs8::{DecodePrivateKey, ObjectIdentifier, PrivateKeyInfo};
use rsa::RsaPrivateKey;

pub struct KeyParser;

impl KeyParser {
    pub fn parse_private_key_from_base64(base64_str: &str) -> Result<RsaPrivateKey> {
        let der_bytes = general_purpose::STANDARD
            .decode(base64_str)
            .map_err(|e| anyhow::anyhow!("base64 decode error: {}", e))?;

        let info = PrivateKeyInfo::from_der(&der_bytes)
            .map_err(|e| anyhow::anyhow!("parse PKCS#8 error: {}", e))?;
        let oid = info.algorithm.oid;

        let rsa_oid = ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.1"); // RSA

        if oid == rsa_oid {
            let key = RsaPrivateKey::from_pkcs8_der(&der_bytes)
                .map_err(|e| anyhow::anyhow!("parse RSA private key error: {}", e))?;
            Ok(key)
        } else {
            bail!("unsupported algorithm OID: {}", oid)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::rsa_utils::{sign_data, verify_signature};

    #[test]
    fn test_parse_private_key_from_base64() {
        let base64_str = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDcJ5eadEaYPF/FhyjVdrN/s/Nw5UmNY781NcV9/7GZkLwNQrO79Uju2fyB5juZcpVZrW8eR/D8fUep3soGocOnYMO4561Xo/j7b/kdc0TLLl09H+GFisAQMky+IHcEp9D81MTEabnJ8IyXA+YYclVR6CWxILR2ZA4qxndos4uzv0vz1N0hY5tzN3E1J6DiaUM+e2QYL6GvCsSQeriQ87Xo8jNliqVDK/+eZU5GEyuHifhZ5dRC59LbhTTsgo7budGTBW1mCnwZ2X0mdYfU/vQvfnAEGJYXvz/lJjW2Y/ABKhXq0DXUiZBUVRLHRmXlFaCeOpeejDxMiEjTXQfIjfbxAgMBAAECggEANMRYlfCgqwRtgA6xPtjAlS8wfjK2umjZ/4rv1w9eJgyGxjbilX5pCLa+yvO//Tt1iJUFOSDNJfdxIcoAai6Dkq6iusLszUDEAKAJ+YETUA/A0VG+4B8tgbRMqJVncXo3oSHuN5WPrlM1n0yT7fAZexRFVHseRfIdYytGm5XNOjuBzzaqghodKGw/IvdADw8eNYBMHUBFjgvjJSVKd99rsiKRRkzIVtBcTs7RxFXVdhB/PhvNxWugb3r3ihX52ho5uAoIHUGE7fzdIPO4iCYv9MRzKuSVwrAPIiJvAP7duwoC/INMFsqdCTuX1NdPS+58Ubgkdkmbf+8BK12dzK8EQQKBgQDtvJL2+j6Nd7rEf0+DK8ENjhWf3ktOoWX6dmepxDrJI1nHulpjZSWy5qmztMGiHF5vU0e7ARZZeFFA4aGXcScTT+9ffuWJP6JA04OJryrWTTEF4qT2aTVHJDOYuOtr6pD7541QdpIVNq6TQu7zITaTBoTbHvNYFVFTdZgi8/Lb9QKBgQDtET57KgDBiedVCwIdODPTTsSGqWzZ5J2qE63CYN4nvkHbtzcPKADnOxq0yaYfrr1olYdqIzi0VWR4bPbvuP7D9jtGbxp7kDI6/ZkdcOIAG/0aFqJaqGGnnzgWQJXv8jn08Z90nrikDrYxBX8U1s/9fMdur//csZmjQPmdBOXtjQKBgQCvhHa4cv61sTypkBijDi2klU7vzc2pis1gggR8uQxxrXC+XZ4YHfgcQeHudDg1OF6cME8YCHB4s7TBgxOrXHXt8ykWRviuQNXIqKBHiZTFzQ2xe6gw6HHWSSryySu+a9qIsGaLjk7B7LIstND3nYDOQZTatdoRIQP+6yXcQGD/9QKBgEyTIlyEP8REOC33JVKs4ciii8Z3mYp0Vx0lyB2eToQF554B+03w/QGzzLeS3w8i0Vmj2x7Ei79sSczAXa8nUVuZAKKKpsI83IzDd57T5JxmbgXsQ7sG4qxTOLmvWP8tfd0J4xi3YCrV+bGx9c+UZ5CYqo6tWPc/gsIB7d7zQxXNAoGAcxv32TAh+eRrVgIC0LMDXyKQ7pKt58RTjL8/SsSwavCKznvAp8S1pEde1/OjUfTiL42muJj1DghytwPIaam57X7/Ikgyz5PxgPzABCWv1BY0P4m37Cv8MYeqKv6e/OtjJs2O+r3GP12SI9RMP1trj7DLt5Z2TUmD5xeDEpdbpbw=";
        let raw_str = "OpenId=HASH13900000002";

        let result = KeyParser::parse_private_key_from_base64(base64_str);
        match result {
            Ok(prv) => {
                println!("Parsed private key: {:?}", prv);
                let pub_key = prv.to_public_key();

                match sign_data(&prv, raw_str) {
                    Ok(sign) => {
                        println!("sign: {}", sign);

                        match verify_signature(&pub_key, raw_str, &sign) {
                            Ok(()) => println!("Signature verification succeeded"),
                            Err(e) => println!("Signature verification failed: {}", e),
                        }
                    }
                    Err(e) => println!("sign_data error: {}", e),
                }
            }
            Err(e) => {
                println!("Failed to parse private key: {}", e);
                return;
            }
        }
    }
}
