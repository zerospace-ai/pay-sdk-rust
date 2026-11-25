# 例とツール

このドキュメントは 2 つの部分に分かれています。
1. **シナリオベースのコード例:** 実際のコードで API 呼び出しと検証を処理する方法を示します。
2. **CLI ツールガイド:** クイックテストのために SDK に含まれているコンパイル済み実行可能ファイルの使用方法について説明します。

---

## 1. シナリオベースのコード例

### 1.1 完全な API 呼び出しと応答の検証

次のコードは、SDK を使用して「ユーザー作成」リクエストを構築し、HTTP リクエストを送信し、プラットフォームから返されたデータの署名のセキュリティを検証する方法を示しています。

```rust
use pay_sdk_rust::{
    response_define::ResponseCommon,
    rsa_utils,
    sdk::{load_config, SDKConfig, Sdk},
};
use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. SDKの初期化
    let cfg = load_config("./config.yaml")?;
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 2. リクエストパラメータと署名ヘッダーの生成
    let open_id = "PT00001";
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // 3. HTTPリクエストの送信
    let client = Client::new();
    let url = "https://sandbox-api.privatex.io/sdk/user/create";
    let resp = client.post(url)
        .header("Content-Type", "application/json")
        .header("key", sdk.config.api_key.clone())
        .header("timestamp", timestamp)
        .header("sign", sign)
        .header("clientSign", client_sign)
        .body(req_body)
        .send()
        .await?;

    let body = resp.text().await?;
    
    // 4. レスポンス署名の自動検証
    let parsed: ResponseCommon = serde_json::from_str(&body)?;
    if parsed.code != 1 {
        println!("リクエスト失敗: {}", parsed.msg);
        return Ok(());
    }

    let map_data = rsa_utils::to_string_map(&body)?;
    let rsp_data: pay_sdk_rust::response_define::ResponseCreateUser = serde_json::from_str(&body)?;
    
    match sdk.verify_rsa_signature(&map_data, &rsp_data.sign) {
        Ok(_) => println!("✅ リクエスト成功および検証完了！返されたOpenId: {}", rsp_data.data.open_id),
        Err(e) => println!("❌ 署名検証失敗: {:?}", e),
    }

    Ok(())
}
```


---

## 2. CLI ツールの使用ガイド

SDK には、各 API エンドポイントをすばやくテストするためのコマンドラインインターフェース (CLI) バイナリファイルが用意されています。

### 2.1 実行可能ファイルのコンパイル

SDK のルートディレクトリで `cargo build --examples` コマンドを実行すると、システムは `target/debug/examples/` ディレクトリの各機能のバイナリ実行可能ファイルを生成します。
* **Windows:** `.exe` で終わるファイルを生成します (例: `create_user.exe`)。
* **Mac/Linux:** 拡張子のないファイルを生成します (例: `create_user`)。

### 2.2 設定ファイルの準備

ツールを実行する前に、設定済みの `config.yaml` ファイルが実行ディレクトリ（通常はプロジェクトのルートディレクトリまたは `target/debug/examples/`）に配置されていることを確認してください。

### 2.3 エンドポイントコマンドのテスト

#### 新規ユーザーの登録
1. `config.yaml` の `UserOpenId` フィールドを変更します。
2. `cargo run --example create_user_client` を実行します。
3. OpenId がすでに登録されている場合、ツールはエラーを返します。

#### ウォレットの登録
1. `config.yaml` で `UserOpenId` と `ChainID` を指定します。
2. `cargo run --example create_wallet` を実行します。

#### 入金アドレスの取得
1. `config.yaml` で、`UserOpenId` とクエリする `ChainIDs` (例: "1,56") を指定します。
2. `cargo run --example get_wallet_addresses` を実行します。

#### 出金の申請
1. `config.yaml` で次を指定します。
   * `UserOpenId`
   * `TokenId`
   * `Amount`
   * `AddressTo`
   * `SafeCheckCode` (一意の注文重複防止コード)
   * `CallbackUrl`
2. `cargo run --example user_withdraw_by_open_id` を実行します。

#### レジ注文の作成
1. `config.yaml` で `OutOrderNo`, `OutUserId`（固定アドレスモード時は必須）, `TokenId`, `Quantity`, `NotifyUrl`, `OrderDetail` を指定します。
2. `cargo run --example new_order` を実行します。

#### ウォレット残高照会
1. `config.yaml` で `WalletAddress`, `ContractAddress`, `WalletBalanceChainId` を指定します。
2. `cargo run --example wallet_balance` を実行します。