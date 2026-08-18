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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseNewOrderData {
    #[serde(rename = "orderNo")]
    pub order_no: String,
    #[serde(rename = "outOrderNo")]
    pub out_order_no: String,
    #[serde(rename = "outUserId")]
    pub out_user_id: String,
    #[serde(rename = "tokenId")]
    pub token_id: i32,
    pub quantity: String,
    pub addresses: String,
    #[serde(rename = "cashierUrl")]
    pub cashier_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseNewOrder {
    pub code: i32,
    pub msg: String,
    pub timestamp: String,
    pub sign: String,
    pub data: ResponseNewOrderData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestCashierCb {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "outOrderId")]
    pub out_order_id: String,
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "exchangeAddress")]
    pub exchange_address: String,
    pub quantity: String,
    #[serde(rename = "quantityPaid")]
    pub quantity_paid: String,
    #[serde(rename = "totalAmount")]
    pub total_amount: String,
    #[serde(rename = "totalAmountPaid")]
    pub total_amount_paid: String,
    #[serde(rename = "outUserId")]
    pub out_user_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub timestamp: String,
    pub sign: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseCashierCb {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseWalletBalance {
    pub code: i32,
    pub msg: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<String>,
    #[serde(default)]
    pub data: Option<String>,
}


