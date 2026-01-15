use alloy::providers::{Provider, ProviderBuilder};
use alloy_primitives::Address;
use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    interface ERC20 {
        function name() view returns (string);
        function symbol() view returns (string);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let  rpc = "https://arbitrum-sepolia-rpc.publicnode.com";
    let provider = ProviderBuilder::new()
        .connect(rpc).await.expect("failed to connect");
    //合约地址
    let contract_address:Address = "0xF41561BF42418B69791f026a97CF9e4F8BC95703".parse().expect("should parse address");
    //绑定合约
    let erc20 = ERC20::new(contract_address, provider);
    let name = erc20.name().call().await?;
    let symbol = erc20.symbol().call().await?;

    println!("name     : {}", name);
    println!("symbol   : {}", symbol);

    Ok(())

}