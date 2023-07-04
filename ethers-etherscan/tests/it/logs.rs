use crate::*;
use ethers_etherscan::logs::LogListParams;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn get_logs_success() {
    run_with_client(Chain::Mainnet, |client| async move {
        let logs = client
            .get_logs(
                Some("0x00000000219ab540356cBB839Cbe05303d7705Fa".parse().unwrap()),
                Some(LogListParams { page: 2, offset: 100, ..Default::default() }),
            )
            .await;
        logs.unwrap();
    })
    .await
}
