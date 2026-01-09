
use alloy::{primitives::address, providers::ProviderBuilder, sol};
use std::error::Error;


sol! { 
   // The `rpc` attribute enables contract interaction via the provider.
   #[sol(rpc)] 
   contract HelloWeb3 { 
        function hello_web3() pure public returns (string money); 
   } 
} 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url =  "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    // Initialize the provider.
    let provider = ProviderBuilder::new().connect_http(rpc_url);
 
    // Instantiate the contract instance.
    let weth = address!("0x3f1f78ed98cd180794f1346f5bd379d5ec47de90");
    let contract = HelloWeb3::new(weth, provider); 
  
    let message = contract.hello_web3().call().await?; 
 
    println!("contract message: {message}");
 
    Ok(())
}
