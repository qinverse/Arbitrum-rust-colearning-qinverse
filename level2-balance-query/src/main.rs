//! Demonstrates reading a contract by fetching the WETH balance of an address.
use alloy::{primitives::address, providers::{Provider, ProviderBuilder}, sol};
use std::error::Error;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the provider.
    let provider = ProviderBuilder::new().connect("https://arbitrum-sepolia-rpc.publicnode.com").await?;

 
    // Fetch the balance of WETH for a given address.
    let owner = address!("0xe3a6E3935E65613C7DE0DB4586dcc91a32A03c41"); 
    let balance = provider.get_balance(owner).await?; 

     // Convert to u128
    let  raw_u128: u128 = balance.to();

    // 转换成人类可读格式eth
    let readable = raw_u128 as f64 / 1e18f64;
  // Convert wei into ETH (18 decimals).
    println!("WETH Balance of {owner}: {readable}");
 
    Ok(())
}