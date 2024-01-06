//!

use ethers::types::{transaction::eip2718::TypedTransaction, TransactionRequest};
use ethers::utils::parse_ether;
use kurtosis_test::{TestEOA, KurtosisTestNetwork, utils, constants};

async fn setup_network() -> KurtosisTestNetwork {
    KurtosisTestNetwork::setup(None).await.unwrap()
}

#[tokio::test]
async fn test_something() {
    let network = setup_network().await;

    // Fetch node execution layer RPC port from network
    let rpc_port = utils::get_el_rpc_port(&network).unwrap();

    // Create EOA to receive funds
    let receiver = TestEOA::new(&network, None).await.unwrap();

    // Create EOA to send funds
    let transfer_amount = parse_ether("1").unwrap();
    let funding_eth = parse_ether("100").unwrap();
    let mut sender = TestEOA::new(&network, Some(funding_eth)).await.unwrap();

    // TODO: network.wait_for_new_block(&rpc_port).await.unwrap();

    // Send funds from sender to receiver
    let tx = TypedTransaction::Legacy(
        TransactionRequest {
            from: Some(sender.address()),
            to: Some(receiver.address().into()),
            gas: Some(constants::ETH_TRANSFER_GAS_LIMIT.into()),
            gas_price: None,
            value: Some(transfer_amount),
            data: None,
            nonce: Some(sender.nonce().into()),
            chain_id: Some(network.chain_id().into()),
        }
    );
    network.send_transaction(&mut sender, &tx, Some(rpc_port)).await.unwrap();

    // TODO: Assert transfer was successful with expected amount
}
