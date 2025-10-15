use crate::key_parser::KeyParser;
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use digest::Digest;
use md5::Md5;
use rand::thread_rng;
use rsa::{pkcs1v15::Pkcs1v15Sign, pkcs8::DecodePublicKey, RsaPrivateKey, RsaPublicKey};
use serde_json::Value;
use std::collections::HashMap;

pub fn compose_params(params: &HashMap<String, String>) -> String {
    let mut keys: Vec<_> = params.keys().filter(|k| *k != "sign").collect();
    keys.sort();
    keys.iter()
        .enumerate()
        .map(|(i, k)| {
            if i > 0 {
                format!("&{}={}", k, params[*k])
            } else {
                format!("{}={}", k, params[*k])
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn load_private_key_from_base64(base64_str: &str) -> Result<RsaPrivateKey> {
    KeyParser::parse_private_key_from_base64(base64_str)
}

pub fn parse_public_key_from_base64(base64_str: &str) -> Result<RsaPublicKey> {
    let der_bytes = general_purpose::STANDARD.decode(base64_str)?;
    let key = RsaPublicKey::from_public_key_der(&der_bytes)?;
    Ok(key)
}

pub fn sign_data(key: &RsaPrivateKey, data: &str) -> Result<String> {
    let mut rng = thread_rng();
    let mut hasher = Md5::new();
    hasher.update(data);
    let hashed_data = hasher.finalize();
    let signature = key.sign_with_rng(&mut rng, Pkcs1v15Sign::new::<Md5>(), &hashed_data)?;
    Ok(general_purpose::STANDARD.encode(signature))
}

pub fn verify_signature(public_key: &RsaPublicKey, data: &str, signature: &str) -> Result<()> {
    let sig_bytes = general_purpose::STANDARD.decode(signature.trim())?;
    let mut hasher = Md5::new();
    hasher.update(data);
    let hashed_data = hasher.finalize();

    let padding = Pkcs1v15Sign::new::<md5::Md5>();
    match public_key.verify(padding, &hashed_data, &sig_bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!("verify fail: {}", e)),
    }
}

pub fn to_string_map(
    jstr: &str,
) -> Result<std::collections::HashMap<String, String>, serde_json::Error> {
    let json_value: Value = serde_json::from_str(jstr)?;
    let mut res_map = std::collections::HashMap::new();

    if let Value::Object(obj) = json_value {
        for (k, v) in obj {
            res_map.insert(k, value_as_string(&v));
        }
    }

    Ok(res_map)
}

pub fn value_as_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "".to_string(),
        Value::Object(map) => {
            
            let mut sorted_keys: Vec<&String> = map.keys().collect();
            sorted_keys.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

            let mut result = String::new();
            for k in sorted_keys {
                if let Some(v) = map.get(k) {
                    result.push_str(&value_as_string(v));
                }
            }
            result
        }
        Value::Array(arr) => {
            let mut result = String::new();
            for item in arr {
                result.push_str(&value_as_string(item));
            }
            result
        }
    }
}
