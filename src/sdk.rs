use crate::request_define::{
    RequestCreateUser, RequestCreateWallet, RequestGetWalletAddresses, RequestNewOrder,
    RequestWalletBalance, RequestWithdrawByOpenID,
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
    /// API Key
    #[serde(rename = "ApiKey")]
    pub api_key: String,

    /// API Secret (sensitive information)
    #[serde(rename = "ApiSecret")]
    pub api_secret: String,

    /// Platform Public Key (Base64 encoded PKCS#8 format)
    #[serde(rename = "PlatformPubKey")]
    pub platform_pub_key: String,

    /// Platform Risk Control Public Key (Base64 encoded PKCS#8 format)
    #[serde(rename = "PlatformRiskPubKey")]
    pub platform_risk_pub_key: String,

    /// User RSA Private Key (Base64 encoded PKCS#8 format)
    #[serde(rename = "RsaPrivateKey")]
    pub rsa_private_key: String,

    /// User Open ID
    #[serde(rename = "UserOpenId", default)]
    pub user_open_id: String,

    /// Blockchain Network ID
    #[serde(rename = "ChainID", default)]
    pub chain_id: String,

    /// Token ID
    #[serde(rename = "TokenId", default)]
    pub token_id: String,

    /// Transaction amount (string format, may need further parsing to number)
    #[serde(rename = "Amount", default)]
    pub amount: String,

    /// Target address
    #[serde(rename = "AddressTo", default)]
    pub address_to: String,

    /// Security check code
    #[serde(rename = "SafeCheckCode", default)]
    pub safe_check_code: String,

    /// Callback URL
    #[serde(rename = "CallbackUrl", default)]
    pub callback_url: String,

    /// Blockchain Network ID list (comma-separated or JSON array)
    #[serde(rename = "ChainIDs", default)]
    pub chain_ids: String,

    /// External order number (used for cashier and other new interfaces)
    #[serde(rename = "OutOrderNo", default)]
    pub out_order_no: String,

    /// Merchant-side user ID (required in fixed-address mode; omit in address-pool mode)
    #[serde(rename = "OutUserId", default)]
    pub out_user_id: String,

    /// Quantity (used for cashier and other new interfaces)
    #[serde(rename = "Quantity", default)]
    pub quantity: f64,

    /// Async callback notification URL (used for cashier and other new interfaces)
    #[serde(rename = "NotifyUrl", default)]
    pub notify_url: String,

    /// Order description (optional)
    #[serde(rename = "OrderDetail", default)]
    pub order_detail: String,

    /// Wallet address
    #[serde(rename = "WalletAddress", default)]
    pub wallet_address: String,

    /// Contract address / Token symbol
    #[serde(rename = "ContractAddress", default)]
    pub contract_address: String,

    /// Query wallet balance chain ID
    #[serde(rename = "WalletBalanceChainId", default)]
    pub wallet_balance_chain_id: i64,
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

    pub fn new_order(
        &self,
        out_order_no: &str,
        token_id: i32,
        quantity: f64,
        notify_url: Option<String>,
        out_user_id: Option<String>,
        order_detail: Option<String>,
    ) -> Result<(String, String, String, String)> {
        let req = RequestNewOrder {
            out_order_no: out_order_no.to_string(),
            out_user_id: out_user_id.filter(|s| !s.is_empty()),
            token_id,
            quantity,
            notify_url: notify_url.filter(|s| !s.is_empty()),
            order_detail: order_detail.filter(|s| !s.is_empty()),
        };
        self.sign_pack(req)
    }

    pub fn new_order_with_request(
        &self,
        req: RequestNewOrder,
    ) -> Result<(String, String, String, String)> {
        self.sign_pack(req)
    }

    pub fn get_wallet_balance(
        &self,
        address: &str,
        contract_address: &str,
        chain_id: i64,
    ) -> Result<(String, String, String, String)> {
        let req = RequestWalletBalance {
            address: address.to_string(),
            contract_address: contract_address.to_string(),
            chain_id,
        };
        self.sign_pack(req)
    }

    pub fn wallet_balance(
        &self,
        address: &str,
        contract_address: &str,
        chain_id: i64,
    ) -> Result<(String, String, String, String)> {
        self.get_wallet_balance(address, contract_address, chain_id)
    }

    fn sign_pack<T: serde::Serialize>(&self, req: T) -> Result<(String, String, String, String)> {
        let timestamp = Utc::now().timestamp_millis().to_string();
        self.sign_pack_with_timestamp(req, &timestamp)
    }

    pub fn sign_pack_with_timestamp<T: serde::Serialize>(
        &self,
        req: T,
        timestamp: &str,
    ) -> Result<(String, String, String, String)> {
        let j_str = serde_json::to_string(&req)?;
        let map_data = rsa_utils::to_string_map(&j_str)?;
        let data_str = rsa_utils::compose_params(&map_data);
        let sign = self.generate_md5_sign(&data_str, timestamp);
        let client_sign = if self.config.rsa_private_key.is_empty() {
            String::new()
        } else {
            self.generate_rsa_signature(&map_data)?
        };
        Ok((j_str, timestamp.to_string(), sign, client_sign))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response_define::RequestCashierCb;
    use base64::{engine::general_purpose, Engine as _};
    use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
    use rsa::RsaPrivateKey;

    fn make_test_sdk() -> Sdk {
        let mut rng = rand::thread_rng();
        let priv_key = RsaPrivateKey::new(&mut rng, 2048).expect("generate rsa key");
        let priv_der = priv_key.to_pkcs8_der().expect("encode pkcs8");
        let pub_der = priv_key
            .to_public_key()
            .to_public_key_der()
            .expect("encode spki");
        Sdk::new(SDKConfig {
            api_key: "your_api_key".to_string(),
            api_secret: "your_api_secret".to_string(),
            platform_pub_key: general_purpose::STANDARD.encode(pub_der.as_bytes()),
            platform_risk_pub_key: String::new(),
            rsa_private_key: general_purpose::STANDARD.encode(priv_der.as_bytes()),
        })
    }

    #[test]
    fn test_new_order_official_md5_vector() {
        let sdk = make_test_sdk();
        let req = RequestNewOrder {
            out_order_no: "1234623".to_string(),
            out_user_id: Some("user_10086".to_string()),
            token_id: 6,
            quantity: 0.01,
            notify_url: Some("https://your-domain.com/callback".to_string()),
            order_detail: Some("VIP会员充值".to_string()),
        };

        let (body, ts, sign, client_sign) = sdk
            .sign_pack_with_timestamp(req, "1770606706659")
            .expect("sign_pack");
        assert_eq!(ts, "1770606706659");
        assert_eq!(sign, "1b5b885633560f447f38073f964ad51b");

        let map_data = rsa_utils::to_string_map(&body).expect("to_string_map");
        assert_eq!(
            rsa_utils::compose_params(&map_data),
            "notifyUrl=https://your-domain.com/callback&orderDetail=VIP会员充值&outOrderNo=1234623&outUserId=user_10086&quantity=0.01&tokenId=6"
        );
        sdk.verify_rsa_signature(&map_data, &client_sign)
            .expect("verify clientSign");
    }

    #[test]
    fn test_new_order_address_pool_mode_omits_optional_fields() {
        let sdk = make_test_sdk();
        let (body, ts, sign, client_sign) = sdk
            .new_order(
                "1234623",
                6,
                0.01,
                Some(String::new()),
                None,
                Some(String::new()),
            )
            .expect("new_order minimal");

        let val: serde_json::Value = serde_json::from_str(&body).expect("parse json");
        let obj = val.as_object().expect("json object");
        assert_eq!(obj.len(), 3);
        assert!(!obj.contains_key("outUserId"));
        assert!(!obj.contains_key("notifyUrl"));
        assert!(!obj.contains_key("orderDetail"));

        let map_data = rsa_utils::to_string_map(&body).expect("to_string_map");
        let expected_md5 = sdk.generate_md5_sign("outOrderNo=1234623&quantity=0.01&tokenId=6", &ts);
        assert_eq!(sign, expected_md5);
        sdk.verify_rsa_signature(&map_data, &client_sign)
            .expect("verify minimal clientSign");
    }

    #[test]
    fn test_cashier_callback_deserialize_with_tx_hash() {
        let raw = r#"{
            "orderId": "D20261116121329000123",
            "outOrderId": "1234623",
            "outUserId": "user_10086",
            "orderType": "1",
            "chainId": "3",
            "tokenId": "6",
            "quantity": "0.0100000000",
            "quantityPaid": "0.0100000000",
            "totalAmount": "0.0099900000",
            "totalAmountPaid": "0.0099900000",
            "exchangeAddress": "TQ5xAddress",
            "orderStatus": "2",
            "txHash": "0xabc123",
            "createdAt": "Mon Nov 16 12:13:29 CST 2026",
            "timestamp": "1770607000000",
            "sign": "sig"
        }"#;
        let cb: RequestCashierCb = serde_json::from_str(raw).expect("deserialize cb");
        assert_eq!(cb.tx_hash, "0xabc123");
        assert_eq!(cb.order_status, "2");
        assert_eq!(cb.out_user_id, "user_10086");
    }
}
