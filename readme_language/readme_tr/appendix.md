# Ek

- [🧩 ChainID Listesi](#-chainid-listesi)
- [🧩 Token Türü](#-token-türü)
- [🌐 Genel Bilgiler](#-genel-bilgiler)
- [🔰 Token Temel Bilgileri](#-token-temel-bilgileri)

## 🧩 ChainID Listesi

| Para Adı | Tam Adı | Blockchain Tarayıcı Adresi | Zincir ID |
| :------------ | :------------------ | :------------------------------ | :----------- |
| eth | eth | https://etherscan.io | 1 |
| trx | Tron | https://tronscan.io | 2 |
| btc | btc | https://blockchair.com/bitcoin | 3 |
| sol | solana | https://explorer.solana.com | 4 |
| xrp | xrp | https://xrpscan.com | 5 |
| eth_optimism | optimism | https://optimistic.etherscan.io | 10 |
| bnb | bnb | https://bscscan.com | 56 |
| matic_polygon | MATIC polygon zinciri | https://polygonscan.com | 137 |
| TON | Toncoin | https://tonscan.org/ | 15186 |

## 🧩 Token Türü

| TokenID | Değer | Açıklama |
| :------ | :------------ | :------------------------------- |
| 1 | ETH-ETH | ETH Ağı ETH |
| 2 | ETH-USDT | ETH Ağı USDT |
| 3 | TRON-TRX | TRON Ağı TRX |
| 4 | TRON-USDT | TRON Ağı Token: USDT |
| 5 | BNB-BNB | BNB Akıllı Zincir Ağı BNB |
| 6 | BNB-USDT | BNB Akıllı Zincir Ağı Token: USDT |
| 11 | Polygon-MATIC | Polygon Ağı Matic |
| 12 | Polygon-USDT | Polygon Ağı Token: USDT |
| 13 | Polygon-USDC | Polygon Ağı Token: USDC |
| 22 | BNB-USDC | BNB Akıllı Zincir Ağı Token: USDC |
| 23 | BNB-DAI | BNB Akıllı Zincir Ağı Token: DAI |
| 24 | ETH-USDC | ETH Ağı USDC |
| 25 | ETH-DAI | ETH Ağı DAI |
| 130 | Optimism-ETH | Optimism Ağı ETH |
| 131 | Optimism-WLD | Optimism Ağı Token: WLD |
| 132 | Optimism-USDT | Optimism Ağı Token: USDT |
| 100 | BTC-BTC | BTC Ağı BTC Ana Zincir Token |
| 200 | TON-TON | TON Ağı TON Ana Zincir Token |

## 🌐 Genel Bilgiler

| Ad | Tür | Örnek | Açıklama |
| :--------- | :-------- | :--------------------------------- | :--------------------------------- |
| Genel Durum Kodu | integer | 1 | 1 başarıyı belirtir. Ayrıntılar için Genel Durum Koduna bakın. |
| Mesaj | string | ok | Metin bilgisi döndürür. |
| Veri | json | {"OpenID":"HEX..."} | Belirli veri içeriğini döndürür. |
| Zaman | timeStamp | 1722587274000 | UTC zamanı (zaman dilimsiz, milisaniye cinsinden). |
| İmza | sign | 9e0ccfe3915e94bcc5bfbBsC5EUxV6 ... | Platform tüm verileri RSA ile imzalar. |

## 🔰 Token Temel Bilgileri

| Ana Zincir Ağı | chain_id [ana zincir ID] | token_id [benzersiz ID] | token_address [sözleşme adresi] | symbol[Token kısaltması] | decimals[Ondalık basamak] |
| --------------- | ------------------ | ------------------ | ---------------------------------------------------------------- | ----------------- | ---------------- |
| Ethereum | 1 | 2 | 0xdac17f958d2ee523a2206206994597c13d831ec7 | USDT | 6 |
| | 1 | 140 | 0x6982508145454Ce325dDbE47a25d4ec3d2311933 | PEPE | 18 |
| | 1 | 141 | 0xb131f4A55907B10d1F0A50d8ab8FA09EC342cd74 | MEME | 18 |
| | 1 | 64 | 0xEd04915c23f00A313a544955524EB7DBD823143d | ACH | 8 |
| | 1 | 25 | 0x6B175474E89094C44Da98b954EedeAC495271d0F | DAI | 18 |
| | 1 | 24 | 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 | USDC | 6 |
| | 1 | 142 | 0x163f8C2467924be0ae7B5347228CABF260318753 | WLD | 18 |
| | | 1 | 1 | ETH | 18 |
| Tron | 2 | 40 | THb4CqiFdwNHsWsQCs4JhzwjMWys4aqCbF | ETH | 18 |
| | 2 | 90 | TPYmHEhy5n8TCEfYGqW2rPxsghSfzghPDn | USDD | 18 |
| | 2 | 26 | TEkxiTehnzSmSe2XqrBj4w32RUN966rdz8 | USDC | 6 |
| | 2 | 33 | TSkW873XMKiDCxGZrA4YH8KGeipLdC6Gyu | CVNT | 18 |
| | 2 | 3 | TRX | TRX | 6 |
| | 2 | 4 | TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t | USDT | 6 |
| Bitcoin | 3 | 100 | BTC | BTC | 8 |
| | 3 | 102 | SATS | SATS | 18 |
| | 3 | 103 | RATS | RATS | 18 |
| | 3 | 101 | ORDI | ORDI | 18 |
| Solana | 4 | 400 | Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB | USDT | 6 |
| | 4 | 401 | EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v | USDC | 6 |
| | 4 | 19 | SOL | SOL | 9 |
| | 4 | 410 | nQ1qgSpXWi71twnWPFjyfCtcbUXbVyQb64RfHKwRpKE | DAOT | 9 |
| XRP | 5 | 200 | XRP | XRP | 6 |
| DogeCoin | 9 | 300 | DOGE | DOGE | 8 |
| Optimistic | 10 | 131 | 0xdC6fF44d5d932Cbd77B52E5612Ba0529DC6226F1 | WLD | 18 |
| | 10 | 130 | ETH | ETH | 18 |
| | 10 | 133 | 0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85 | USDC | 6 |
| | 10 | 132 | 0x94b008aA00579c1307B0EF2c499aD98a8ce58e58 | USDT | 6 |
| Bnb Smart Chain | 56 | 62 | 0xc0be866ecc026957fc7160c1a45f2bee9870fd46 | ARK | 18 |
| | 56 | 561 | 0xbA2aE424d960c26247Dd6c32edC70B295c744C43 | DOGE | 8 |
| | 56 | 68 | 0x6FDcdfef7c496407cCb0cEC90f9C5Aaa1Cc8D888 | VET | 18 |
| | 56 | 63 | 0x8540f3D726Aed340Bc57Fd07a61b0ae2a9d5ECa9 | PUC | 18 |
| | 56 | 65 | 0xbc7d6b50616989655afd682fb42743507003056d | ACH | 8 |
| | 56 | 66 | 0xFE8bF5B8F5e4eb5f9BC2be16303f7dAB8CF56aA8 | BIBI | 18 |
| | 56 | 29 | 0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56 | BUSD | 18 |
| | 56 | 31 | 0x7130d2A12B9BCbFAe4f2634d864A1Ee1Ce3Ead9c | BTCB | 18 |
| | 56 | 30 | 0x2170Ed0880ac9A755fd29B2688956BD959F933F8 | ETH | 18 |
| | 56 | 6 | 0x55d398326f99059ff775485246999027b3197955 | USDT | 18 |
| | 56 | 23 | 0x1AF3F329e8BE154074D8769D1FFa4eE058B1DBc3 | DAI | 18 |
| | 56 | 22 | 0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d | USDC | 18 |
| | 56 | 5 | BNB | BNB | 18 |
| Polygon | 137 | 12 | 0xc2132D05D31c914a87C6611C10748AEb04B58e8F | USDT | 6 |
| | 137 | 13 | 0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174 | USDC | 6 |
| | 137 | 11 | POL | MATIC | 18 |
| | 137 | 110 | 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 | USDC | 6 |
| CVN Chain | 2032 | 7 | CVN | CVN | 18 |
| | 2032 | 35 | 0x109B57A29eE6E9A93f33687F6CE553fB18D8EE78 | USDT | 6 |
| | 2032 | 51 | 0x6b94b0a2878c68811c1bd6cecc2b7cc44a9ed7ab | HPT | 8 |
| Merlin | 4200 | 500 | BTC | BTC | 18 |
| | 4200 | 501 | 0x5c46bFF4B38dc1EAe09C5BAc65872a1D8bc87378 | MERL | 18 |
| Base | 8453 | 801 | 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913 | USDC | 6 |
| | 8453 | 802 | ETH | ETH | 18 |
| TON | 201 | 0: | 105e5589bc66db15f13c177a12f2cf3b94881da2f4b8e7922c58569176625eb5 | JETTON | 9 |
| 15186 | 202 | 0: | b113a994b5024a16719f69139328eb759596c38a25f59028b146fecdc3621dfe | USDT | 6 |
| | 15186 | 200 | TON | TON | 9 |
| Arbitrum One | 42161 | 122 | 0xaf88d065e77c8cC2239327C5EDb3A432268e5831 | USDC | 6 |
| | 42161 | 121 | 0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9 | USDT | 6 |
| | 42161 | 120 | ETH | ETH | 18 |
| | 42161 | 123 | 0x9fE175843Df9deCd99C78E72b2424C47D61Ad2bF | ATM | 18 |
| | 42161 | 124 | 0x58BDf739aE17d1C60C6FD3433E288E38B81C2853 | SAM | 18 |
| Avax Chain C | 43114 | 18 | 0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E | USDC | 6 |
| | 43114 | 17 | 0xc7198437980c041c805A1EDcbA50c1Ce5db95118 | USDT | 6 |
| | 43114 | 16 | AVAX | AVAX | 18 |
| NA Chain | 65143 | 600 | NAC | NAC | 9 |
| | 65143 | 601 | GAT | GAT | 9 |
| ODIN | 666666 | 80 | ODIN | ODIN | 18 |
| THOR | 868868 | 81 | THOR | THOR | 18 |