mod init;

use init::*;

use color_eyre::*;
use haqq_grpc::cosmos::bank::v1beta1 as bank;
use haqq_grpc::haqq::vesting::v1 as vesting;
use tonic::transport::Channel;
// use tonic::transport::ClientTlsConfig;

#[tokio::test]
async fn test_grpc() -> Result<()> {
    init();

    let node_url = std::env::var("COSMOS_GRPC_URL")?;
    // let tls = ClientTlsConfig::default();
    let channel = Channel::from_shared(node_url.clone())?
        // .tls_config(tls)?
        .connect()
        .await?;

    let addr = "haqq1jy4rhr8kqr9a6u0lcs3yaqzszgs5sg6x38xqds".to_string();

    let mut client = vesting::query_client::QueryClient::new(channel.clone());

    // Haqq Vesting Balance

    let rsp = client
        .balances(
            vesting::QueryBalancesRequestBuilder::default()
                .address(addr.clone())
                .build()?,
        )
        .await?;

    println!("{:?}", rsp);

    // Cosmos Balance (default pattern)

    let mut client = bank::query_client::QueryClient::new(channel);

    let rsp = client
        .balance(bank::QueryBalanceRequest {
            address: addr.clone(),
            denom: "aISLM".to_string(),
        })
        .await?;

    println!("{:?}", rsp);

    // Cosmos Balance (builder pattern)

    let rsp = client
        .balance(
            bank::QueryBalanceRequestBuilder::default()
                .address(addr)
                .denom("aISLM".to_string())
                .build()?,
        )
        .await?;

    println!("{:?}", rsp);

    Ok(())
}
