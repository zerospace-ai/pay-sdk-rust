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
    pub type_field: String,

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
