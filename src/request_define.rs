use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct RequestCreateUser {
    #[serde(rename = "OpenId")]
    pub open_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateWallet {
    #[serde(rename = "ChainID")]
    pub chain_id: String,
    
    #[serde(rename = "OpenId")]
    pub open_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetWalletAddresses {
    #[serde(rename = "OpenId")]
    pub open_id: String,

    #[serde(rename = "ChainIDs")]
    pub chain_ids: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestTokenCb {
    #[serde(rename = "openid")]
    pub open_id: String,

    #[serde(rename = "totalvalue")]
    pub total_value: String,

    #[serde(rename = "amount")]
    pub amount: String,

    #[serde(rename = "chainid")]
    pub chainid: String,

    #[serde(rename = "confirm")]
    pub confirm: String,

    #[serde(rename = "createdtime")]
    pub created_time: String,

    #[serde(rename = "from")]
    pub from: String,

    #[serde(rename = "hash", skip_serializing_if = "String::is_empty")]
    pub hash: String,

    #[serde(rename = "safecode")]
    pub safecode: String,

    #[serde(rename = "sign")]
    pub sign: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "timestamp")]
    pub timestamp: String,

    #[serde(rename = "to")]
    pub to: String,

    #[serde(rename = "tokenaddress")]
    pub token_address: String,

    #[serde(rename = "tokenid")]
    pub token_id: String,

    #[serde(rename = "type")]
    pub type_field: String, // 'type' is a reserved keyword in Rust, renamed to type_field

    #[serde(rename = "fee")]
    pub fee: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestWithdrawByOpenID {
    #[serde(rename = "OpenId")]
    pub open_id: String,
    #[serde(rename = "TokenId")]
    pub token_id: String,
    #[serde(rename = "Amount")]
    pub amount: String,
    #[serde(rename = "AddressTo")]
    pub address_to: String,
    #[serde(rename = "CallBackUrl")]
    pub callback_url: String,
    #[serde(rename = "SafeCheckCode")]
    pub safe_check_code: String,
}

fn is_none_or_empty(opt: &Option<String>) -> bool {
    opt.as_deref().map_or(true, str::is_empty)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestNewOrder {
    #[serde(rename = "outOrderNo")]
    pub out_order_no: String,
    #[serde(rename = "outUserId", default, skip_serializing_if = "is_none_or_empty")]
    pub out_user_id: Option<String>,
    #[serde(rename = "tokenId")]
    pub token_id: i32,
    pub quantity: f64,
    #[serde(rename = "notifyUrl", default, skip_serializing_if = "is_none_or_empty")]
    pub notify_url: Option<String>,
    #[serde(rename = "orderDetail", default, skip_serializing_if = "is_none_or_empty")]
    pub order_detail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestWalletBalance {
    pub address: String,
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    #[serde(rename = "chainId")]
    pub chain_id: i64,
}

