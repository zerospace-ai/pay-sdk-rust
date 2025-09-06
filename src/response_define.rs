use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCommon {
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseSuccess {
    pub code: i32,
    pub msg: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    pub sign: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseCreateUserData {
    #[serde(rename = "OpenId")]
    pub open_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseCreateUser {
    pub code: i32,
    pub timestamp: String,
    pub msg: String,
    pub data: ResponseCreateUserData,
    pub sign: String,
}

#[derive(Serialize)]
pub struct ResponseWithdraw {
    pub code: String,
    pub timestamp: String,
    pub message: String,
    pub sign: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseGetWalletAddresses {
    pub msg: String,
    pub code: i64,
    pub data: GetWalletAddressesData,
    pub sign: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetWalletAddressesData {
    #[serde(rename = "Addresses")]
    pub addresses: Vec<GetWalletAddressesAddress>,
    #[serde(rename = "PartnerId")]
    pub partner_id: i64,
    #[serde(rename = "OpenId")]
    pub open_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetWalletAddressesAddress {
    pub address: String,
    #[serde(rename = "chainID")]
    pub chain_id: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseCreateWallet {
    pub sign: String,
    pub timestamp: String,
    pub data: ResponseCreateWalletData,
    pub msg: String,
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseCreateWalletData {
    pub address: String,
    #[serde(rename = "UserId")]
    pub user_id: i32,
    #[serde(rename = "PartnerId")]
    pub partner_id: i32,
    #[serde(rename = "ChainID")]
    pub chain_id: i32,
    #[serde(rename = "OpenId")]
    pub open_id: String,
}
