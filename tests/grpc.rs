mod init;

use init::*;

use ::haqq_proto::cosmos::bank::v1beta1 as bank;
use ::haqq_proto::haqq::vesting::v1 as vesting;
use color_eyre::*;

#[tokio::test]
async fn test_grpc() -> Result<()> {
    init();

    let node_url = std::env::var("COSMOS_GRPC_URL")?;
    let addr = "haqq1jy4rhr8kqr9a6u0lcs3yaqzszgs5sg6x38xqds".to_string();

    let mut client = vesting::query_client::QueryClient::connect(node_url.clone()).await?;

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

    let mut client = bank::query_client::QueryClient::connect(node_url).await?;

    let rsp = client
        .balance(bank::QueryBalanceRequest {
            address: addr.clone(),
            denom: "aISLM".to_string(),
            ..Default::default()
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
