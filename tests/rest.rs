mod init;
use ::haqq_rest::apis::configuration::Configuration;
use ::haqq_rest::apis::query_api;
use color_eyre::*;
use init::*;

#[tokio::test]
async fn test_rest() -> Result<()> {
    init();

    let node_url = std::env::var("COSMOS_REST_URL")?;
    let addr = "haqq1jy4rhr8kqr9a6u0lcs3yaqzszgs5sg6x38xqds";

    let conf = Configuration {
        base_path: node_url,
        ..Default::default()
    };

    // Haqq Vesting Balance

    let balances = query_api::balances(&conf, addr).await?;
    println!("{:?}", balances);

    // Cosmos Balance

    let balances = query_api::all_balances(&conf, addr, None, None, None, None, None).await?;
    println!("{:?}", balances);

    Ok(())
}
