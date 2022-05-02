use std::{env, future, process};
use std::env::VarError;
use web3::futures::StreamExt;
use web3::transports::{WebSocket};

fn get_node_endpoint() -> Result<String, VarError> {
    env::var("NODE_ENDPOINT")
}

#[tokio::main]
async fn main() -> web3::Result {
    let node_endpoint = match get_node_endpoint() {
        Ok(r) => r,
        Err(e) => {
            println!("Failed to start listener: {}", e);
            process::exit(1)
        },
    };

    let sub_transport = WebSocket::new(node_endpoint.as_str()).await?;
    let subscriber = web3::Web3::new(sub_transport);

    /*
    let clt_transport = Http::new(node_endpoint.as_str())?;
    let clt = web3::Web3::new(clt_transport);
     */

    let pending_transactions = subscriber.eth_subscribe().subscribe_new_pending_transactions().await?;

    pending_transactions.for_each(|pending_txn| {
        println!("{}", pending_txn.unwrap());
        future::ready(())
    }).await;

    Ok(())
}
