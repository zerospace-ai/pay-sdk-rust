use crate::request_define::{
    RequestCreateUser, RequestCreateWallet, RequestGetWalletAddresses, RequestWithdrawByOpenID,
};
use crate::rsa_utils;
use anyhow::Result;
use chrono::Utc;
use digest::Digest;
use md5::Md5;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(rename = "ApiKey")]
    pub api_key: String,

    #[serde(rename = "ApiSecret")]
    pub api_secret: String,

    #[serde(rename = "PlatformPubKey")]
    pub platform_pub_key: String,

    #[serde(rename = "PlatformRiskPubKey")]
    pub platform_risk_pub_key: String,

    #[serde(rename = "RsaPrivateKey")]
    pub rsa_private_key: String,

    #[serde(rename = "UserOpenId")]
    pub user_open_id: String,

    #[serde(rename = "ChainID")]
    pub chain_id: String,

    #[serde(rename = "TokenId")]
    pub token_id: String,

    #[serde(rename = "Amount")]
    pub amount: String,

    #[serde(rename = "AddressTo")]
    pub address_to: String,

    #[serde(rename = "SafeCheckCode")]
    pub safe_check_code: String,

    #[serde(rename = "CallbackUrl")]
    pub callback_url: String,

    #[serde(rename = "ChainIDs")]
    pub chain_ids: String,
}
pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let content = fs::read_to_string(path)?;
    let cfg: Config = serde_yaml::from_str(&content)?;
    Ok(cfg)
}

#[derive(Clone)]
pub struct SDKConfig {
    pub api_key: String,
    pub api_secret: String,
    pub platform_pub_key: String,
    pub platform_risk_pub_key: String,
    pub rsa_private_key: String,
}

#[derive(Clone)]
pub struct Sdk {
    pub config: SDKConfig,
}

impl Sdk {
    pub fn new(config: SDKConfig) -> Self {
        Self { config }
    }

    pub fn generate_md5_sign(&self, data: &str, timestamp: &str) -> String {
        let to_hash = format!("{}{}{}", self.config.api_secret, data, timestamp);

        let mut hasher = Md5::new();
        hasher.update(to_hash);
        let hashed_data = hasher.finalize();

        format!("{:x}", hashed_data)
    }

    pub fn generate_rsa_signature(&self, map_data: &HashMap<String, String>) -> Result<String> {
        let raw = rsa_utils::compose_params(map_data);
        let key = rsa_utils::load_private_key_from_base64(&self.config.rsa_private_key)?;
        rsa_utils::sign_data(&key, &raw)
    }

    pub fn verify_rsa_signature(
        &self,
        map_data: &HashMap<String, String>,
        sign: &str,
    ) -> Result<()> {
        let raw = rsa_utils::compose_params(map_data);
        let pub_key = rsa_utils::parse_public_key_from_base64(&self.config.platform_pub_key)?;
        rsa_utils::verify_signature(&pub_key, &raw, sign)
    }

    pub fn create_user(&self, open_id: &str) -> Result<(String, String, String, String)> {
        let req = RequestCreateUser {
            open_id: open_id.to_string(),
        };
        self.sign_pack(req)
    }

    pub fn create_wallet(
        &self,
        open_id: &str,
        chain_id: &str,
    ) -> Result<(String, String, String, String)> {
        let req = RequestCreateWallet {
            chain_id: chain_id.to_string(),
            open_id: open_id.to_string(),
        };
        self.sign_pack(req)
    }

    pub fn user_withdraw_by_open_id(
        &self,
        open_id: &str,
        token_id: &str,
        amount: &str,
        address_to: &str,
        callback_url: &str,
        safe_check_code: &str,
    ) -> Result<(String, String, String, String)> {
        let req = RequestWithdrawByOpenID {
            open_id: open_id.to_string(),
            token_id: token_id.to_string(),
            amount: amount.to_string(),
            address_to: address_to.to_string(),
            callback_url: callback_url.to_string(),
            safe_check_code: safe_check_code.to_string(),
        };
        self.sign_pack(req)
    }

    pub fn get_wallet_addresses(
        &self,
        open_id: &str,
        chain_ids: &str,
    ) -> Result<(String, String, String, String)> {
        let req = RequestGetWalletAddresses {
            chain_ids: chain_ids.to_string(),
            open_id: open_id.to_string(),
        };
        self.sign_pack(req)
    }

    fn sign_pack<T: serde::Serialize>(&self, req: T) -> Result<(String, String, String, String)> {
        let map_data: HashMap<String, String> =
            serde_json::from_value(serde_json::to_value(&req)?)?;
        let data_str = rsa_utils::compose_params(&map_data);
        let timestamp = Utc::now().timestamp_millis().to_string();
        let sign = self.generate_md5_sign(&data_str, &timestamp);
        let client_sign = self.generate_rsa_signature(&map_data)?;
        let j_str = serde_json::to_string(&req)?;
        Ok((j_str, timestamp, sign, client_sign))
    }
}
