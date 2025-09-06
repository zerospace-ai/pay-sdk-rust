use actix_web::{post, web, App, HttpServer, HttpResponse};
use chrono::Utc;
use std::collections::HashMap;
use pay_sdk_rust::{
    sdk::{Sdk, SDKConfig},
    request_define::RequestTokenCb,
    response_define::ResponseWithdraw,
};

#[post("/withdraw_callback")]
async fn withdraw_callback(sdk: web::Data<Sdk>, body: web::Bytes) -> HttpResponse {
    let body_str = std::str::from_utf8(&body).unwrap_or("");
    let req: RequestTokenCb = match serde_json::from_str(body_str) {
        Ok(r) => r,
        Err(_) => return HttpResponse::BadRequest().body("Invalid JSON"),
    };

    let map_obj: HashMap<String, String> = match serde_json::from_str(body_str) {
        Ok(m) => m,
        Err(_) => return HttpResponse::BadRequest().body("Failed to parse JSON to map"),
    };

    if let Err(e) = sdk.verify_rsa_signature(&map_obj, &req.sign) {
        return HttpResponse::BadRequest().body(format!("verify RSA signature fail: {}", e));
    }

    let timestamp = Utc::now().timestamp().to_string();
    let mut rsp = ResponseWithdraw {
        code: "0".into(),
        timestamp,
        message: "".into(),
        sign: "".into(),
    };

    // 对响应签名
    let rsp_map: HashMap<String, String> = serde_json::from_str(&serde_json::to_string(&rsp).unwrap()).unwrap();
    match sdk.generate_rsa_signature(&rsp_map) {
        Ok(sig) => rsp.sign = sig,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to sign response: {}", e)),
    }

    HttpResponse::Ok().json(rsp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = SDKConfig {
        api_key: "YourApiKey".into(),
        api_secret: "YourApiSecret".into(),
        platform_pub_key: "Base64PlatformPubKey".into(),
        platform_risk_pub_key: "Base64PlatformRiskPubKey".into(),
        rsa_private_key: "Base64RsaPrivateKey".into(),
    };
    let sdk = web::Data::new(Sdk::new(config));

    println!("Starting withdraw callback server on http://127.0.0.1:9003 ...");

    HttpServer::new(move || {
        App::new()
            .app_data(sdk.clone())
            .service(withdraw_callback)
    })
    .bind("127.0.0.1:9003")?
    .run()
    .await
}
