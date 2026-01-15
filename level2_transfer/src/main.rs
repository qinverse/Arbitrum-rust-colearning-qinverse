use alloy::{
    network::{eip2718::Encodable2718, EthereumWallet, TransactionBuilder},
    primitives::{U256, Address},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;
// use dotenvy::dotenv;
use std::{env, str::FromStr};

#[tokio::main]
async fn main() -> Result<()> {
    // ---- 1. 获取私钥
    println!("PRIVATE_KEY = {:?}", std::env::var("PRIVATE_KEY"));

    let pk_str = env::var("PRIVATE_KEY").expect("PRIVATE_KEY环境变量未设置!");
    let pk = PrivateKeySigner::from_str(&pk_str)?;
    let wallet = EthereumWallet::from(pk);

    // ---- 2. 连接 Arbitrum Sepolia
    let rpc = "https://arbitrum-sepolia-rpc.publicnode.com";
    let provider = ProviderBuilder::new().on_http(rpc.parse()?);

    // ---- 3. from / to
    let from = wallet.default_signer().address(); // 从 wallet 获取地址
    let to = Address::from_str("0x2e13e7C90B6d627DbB06768c8018A4fbF030AC9c")?;

    if from == to {
        panic!("接收地址不能与发送地址相同");
    }

    println!("From = {from:?}");
    println!("To   = {to:?}");

    // ---- 4. 金额 0.001 ETH
    let amount = U256::from(10u64.pow(15));

    // ---- 5. 获取 nonce
    let nonce = provider.get_transaction_count(from).await?;

    // ---- 6. 获取 chain_id
    let chain_id = provider.get_chain_id().await?;

    // ---- 7. 获取 gas price
    let gas_price = provider.get_gas_price().await?;

    // ---- 8. 构造交易
    let tx = TransactionRequest::default()
        .with_from(from)
        .with_to(to)
        .with_value(amount)
        .with_nonce(nonce)
        .with_gas_price(gas_price)
        .with_chain_id(chain_id);

    // 估算 gas limit
    let gas_limit = provider.estimate_gas(&tx).await?;
    println!("gas limit: {}", gas_limit);

    // 添加 gas limit (注意这里不要用 mut)
    let tx = tx.with_gas_limit(gas_limit);

    // ---- 9. 构建并签名交易
    let tx_envelope = tx.build(&wallet).await?;

    // ---- 10. 编码为原始交易
    let raw_tx = tx_envelope.encoded_2718();

    // ---- 11. 发送原始交易
    let pending = provider.send_raw_transaction(&raw_tx).await?;

    println!("tx hash = {:?}", pending.tx_hash());

    // ---- 12. 等待确认
    let receipt = pending.get_receipt().await?;
    println!("确认receipt = {receipt:?}");

    Ok(())
}