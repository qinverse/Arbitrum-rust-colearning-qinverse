use alloy::{
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    network::TransactionBuilder,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 连接 Arbitrum Sepolia 公共 RPC
    let rpc = "https://arbitrum-sepolia-rpc.publicnode.com";
    let provider = ProviderBuilder::new().on_http(rpc.parse()?);

    // ******** 1. 获取实时 Gas Price ********
    let gas_price: u128 = provider.get_gas_price().await?;
    println!("Gas Price (wei): {}", gas_price);

    // ******** 2. 构造基础 ETH 转账请求 ********
    let to: Address = "0x000000000000000000000000000000000000dead".parse()?;

    let tx = TransactionRequest::default()
        .with_to(to)
        .with_value(U256::from(1u64)); // 1 wei

    // ******** 3. 动态估算 Gas Limit ********
    let gas_limit: u64 = provider.estimate_gas(&tx).await?;
    println!("Gas Limit: {}", gas_limit);

    // ******** 4. 计算 Gas Fee ********
    let gas_fee_wei = U256::from(gas_price) * U256::from(gas_limit);
    println!("Estimated Gas Fee (wei): {}", gas_fee_wei);

    println!(
        "Estimated Gas Fee (ETH): {:.18} ETH",
        wei_to_eth(gas_fee_wei)
    );

    Ok(())
}

/// wei → ETH 转换
fn wei_to_eth(wei: U256) -> f64 {
    let base = 1_000_000_000_000_000_000_f64; // 1e18
    let v = wei.to::<u128>() as f64;
    v / base
}
