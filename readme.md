# Eversend Rust SDK

[![CI](https://github.com/rnambaale/eversend-rust-sdk/actions/workflows/rust.yml/badge.svg)](https://github.com/rnambaale/eversend-rust-sdk/actions/workflows/rust.yml)
[![Codecov](https://codecov.io/github/rnambaale/eversend-rust-sdk/coverage.svg?branch=master)](https://codecov.io/gh/rnambaale/eversend-rust-sdk)
[![Dependency status](https://deps.rs/repo/github/rnambaale/eversend-rust-sdk/status.svg)](https://deps.rs/repo/github/rnambaale/eversend-rust-sdk)

[Experimental] Rust SDK for interacting with the Eversend API.

## Table of Contents
1. [Installation](#installation)
2. [Initialization](#initialization)
3. [Usage](#usage)

## Installation

```sh
cargo add eversend_rust_sdk
```

Or add the following to your Cargo.toml:

```toml
[dependencies]
eversend_rust_sdk = "0.1"
```

## Initialization
```rust
use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};

let eversend_client = Eversend::new(
    &ClientId::from("clientId"),
    &ClientSecret::from("clientSecret")
);
```

You can get your clientId and clientSecret from the settings section in the [dashboard](https://business.eversend.co/settings)

## Usage
### Wallets

**Get all wallets**
```rust
let wallets = eversend_client
    .wallets()
    .get_wallets()
    .await?;
```

**Get one wallet**
```rust
use eversend_rust_sdk::wallets::types::WalletId;

let wallet = eversend_client
    .wallets()
    .get_wallet(&WalletId::from("UGX"))
    .await?;
```

### Transactions

**Get all transactions**
```rust
use eversend_rust_sdk::transactions::{
    GetTransactionsParams,
    TransactionCurrencyOption,
    TransactionRangeOption,
    TransactionStatusOption,
    TransactionTypeOption
};

let transactions = eversend_client
    .transactions()
    .get_transactions(
        &GetTransactionsParams {
            currency: TransactionCurrencyOption::UGX,
            from: String::from("2024-01-01"),
            to: String::from("2024-01-01"),
            limit: 10,
            page: 1,
            range: TransactionRangeOption::MONTH,
            search: String::from("BE11640235387619"),
            transaction_status: TransactionStatusOption::PENDING,
            transaction_type: TransactionTypeOption::PAYOUT,
        }
    )
    .await?;
```

**Get one transaction**

```rust
use eversend_rust_sdk::transactions::GetTransactionParams;

let transaction = eversend_client
    .transactions()
    .get_transaction(
        &GetTransactionParams {
            transaction_id: String::from("BE11640235387619"),
        }
    )
    .await?;

```

### Exchange

To exchange from one wallet to another, you first have to generate a quotation. This returns a token with a 30s timeout that you can use to make the exchange.

**Get exchange quotation**

```rust
use eversend_rust_sdk::wallets::types::WalletId;
use eversend_rust_sdk::exchange::CreateQuotationParams;

let quotation = eversend_client
    .exchange()
    .create_quotation(&CreateQuotationParams{
        amount: String::from("10.0"),
        from: &WalletId::from("UGX"),
        to: &WalletId::from("KES")
    })
    .await?;
```

**Exchange currency**

```rust
use eversend_rust_sdk::exchange::CreateExchangeParams;

let exchange = eversend_client
    .exchange()
    .create_exchange(&CreateExchangeParams{
        quotation_token: String::from("dhhsggajjshhdhdhd")
    })
    .await?;
```

### Beneficiaries

**Get beneficiaries**

```rust
use eversend_rust_sdk::beneficiaries::GetBeneficiariesParams;

let beneficiaries = eversend_client
    .beneficiaries()
    .get_beneficiaries(&GetBeneficiariesParams::default())
    .await?;

```

The `page` and `limit` parameters default to `1` and `10` respectively.

**Get single beneficiary**

```rust
let beneficiary = eversend_client
    .beneficiaries()
    .get_beneficiary(100)
    .await?;

```

**Create a beneficiary**

```rust
use eversend_rust_sdk::beneficiaries::CreateBeneficaryParams;

let response = eversend_client
    .beneficiaries()
    .create_beneficiary(
        &CreateBeneficaryParams {
            first_name: String::from("Jane"),
            last_name: String::from("Doe"),
            country: String::from("KE"), // Alpha-2 country code
            phone_number: String::from("+254781650002"), // Should be in international format
            bank_account_name: Some(String::from("Stanbic Bank")),
            bank_account_number: Some(String::from("28776353527287")),
            is_bank: true,
            is_momo: true,
        },
    );
```

**Delete a beneficiary**

```rust
let response = eversend_client
    .beneficiaries()
    .delete_beneficiary(
        206
    )
    .await?;

```

### Collections

**Get collection fees**
```rust
use eversend_rust_sdk::collections::{GetCollectionFeesParams, CollectionMethod};

let fees = eversend_client
    .collections()
    .get_collection_fees(
        &GetCollectionFeesParams {
            method: CollectionMethod::MOMO,
            currency: String::from("KES"),
            amount: 1000
        }
    )
    .await?;
```

**Get collection OTP**

>Required when initiating mobile money collections

```rust
use eversend_rust_sdk::collections::GetCollectionOtpParams;

let otp = eversend_client
    .collections()
    .get_collection_otp(
        &GetCollectionOtpParams {
            phone_number: String::from("+256712345678"),
        }
    )
    .await?;
```

**Initiate Mobile Money collection**

```rust
use eversend_rust_sdk::collections::GetMobileMoneyCollectionParams;

let collection = eversend_client
    .collections()
    .get_mobile_money_collection(
        &GetMobileMoneyCollectionParams {
            amount: 1000,
            country: String::from("UG"),
            currency: String::from("UGX"),
            phone_number: String::from("+256712345678"),
            transaction_ref: Some(String::from("ADR234526534")),
            redirect_url: Some(String::from("https://eversend.co")),
            customer: None,
            otp: None,
        }
    )
    .await?;
```

### Payouts

**Get delivery countries**
```rust
let countries = eversend_client
    .payouts()
    .get_delivery_countries()
    .await?;
```

**Get delivery banks**
```rust
let banks = eversend_client
    .payouts()
    .get_delivery_banks(String::from("UG"))
    .await?;

```

**Get payout quotation (Momo and Bank)**

```rust
use eversend_rust_sdk::payouts::CreateMomoAndBankPayoutQuotationParams;

let quotation = eversend_client
    .payouts()
    .create_momo_and_bank_payout_quotation(
        &CreateMomoAndBankPayoutQuotationParams {
            amount: 20,
            amount_type: String::from("SOURCE"),
            destination_country: String::from("KE"),
            destination_currency: String::from("KES"),
            source_wallet: String::from("KES"),
            transaction_type: String::from("momo"),
        }
    )
    .await?;
```

**Get payout quotation (Eversend)**
```rs
use eversend_rust_sdk::payouts::CreateEversendPayoutQuotationParams;

let quotation = eversend_client
    .payouts()
    .create_eversend_payout_quotation(
        &CreateEversendPayoutQuotationParams {
            amount: 20,
            amount_type: String::from("SOURCE"),
            email: String::from("satowind@gmail.com"),
            identifier: String::from("email"),
            phone: String::from("+256789123456"),
            source_wallet: String::from("KES"),
            tag: String::from("the-tag"),
        }
    )
    .await?;
```

**Create Momo payout transaction**

```rust
use eversend_rust_sdk::payouts::CreateMomoPayoutTransactionParams;

let transaction = eversend_client
    .payouts()
    .create_momo_payout_transaction(
        &CreateMomoPayoutTransactionParams {
            country: String::from("UG"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            phone_number: String::from("+256789123456"),
            token: String::from("some-token"),
            transaction_ref: String::from("some-reference")
        }
    )
    .await?;
```

**Create Bank payout transaction**
```rust
use eversend_rust_sdk::payouts::CreateBankPayoutTransactionParams;

let transaction = eversend_client
    .payouts()
    .create_bank_payout_transaction(
        &CreateBankPayoutTransactionParams {
            country: String::from("UG"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            phone_number: String::from("+256789123456"),
            token: String::from("some-token"),
            transaction_ref: String::from("some-reference"),
            bank_account_name: String::from("John Doe"),
            bank_account_number: String::from("12345"),
            bank_code: String::from("1234"),
            bank_name: String::from("World Bank"),
        }
    )
    .await?;
```

**Create Beneficiary payout transaction**

```rust
use eversend_rust_sdk::payouts::CreateBeneficiaryPayoutTransactionParams;

let transaction = eversend_client
    .payouts()
    .create_beneficiary_payout_transaction(
        &CreateBeneficiaryPayoutTransactionParams {
            token: String::from("some-token"),
            beneficiary_id: String::from("123"),
        }
    )
    .await?;
```

**Create Eversend payout transaction**
```rust
use eversend_rust_sdk::payouts::CreateEversendPayoutTransactionParams;

let transaction = eversend_client
    .payouts()
    .create_eversend_payout_transaction(
        &CreateEversendPayoutTransactionParams {
            token: String::from("some-token"),
            transaction_ref: String::from("some-reference"),
        }
    )
    .await?;
```

## Contributing
Contributions are welcome. For more info please read the [Contribution Guideline](CONTRIBUTING.md).
