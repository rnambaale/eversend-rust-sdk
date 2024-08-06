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
use eversend_rust_sdk::{ClientId,Eversend};

let eversend_client = Eversend::new(
    &ClientId::from("clientId"),
    &String::from("clientSecret")
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
...
