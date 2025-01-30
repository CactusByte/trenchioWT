use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use tokio::sync::mpsc;
use tokio::task;
use tokio_tungstenite::connect_async;
use std::collections::HashMap;
use url::Url;
use std::str::FromStr;

use solana_client::{
    pubsub_client::{LogsSubscription, PubsubClient},
    rpc_config::{
        RpcAccountInfoConfig, RpcBlockSubscribeConfig,
        RpcBlockSubscribeFilter, RpcProgramAccountsConfig,
        RpcTransactionLogsConfig, RpcTransactionLogsFilter,
    },
};

#[tokio::main]
async fn main() {
    let rpc_url = "wss://api.mainnet-beta.solana.com";  // Use your Helius RPC or another WebSocket provider
    let program_id = Pubkey::from_str("YourProgramIDHere").unwrap();
    
    // HashMap of wallets to track
    let mut tracked_wallets: HashMap<Pubkey, bool> = HashMap::new();
    tracked_wallets.insert(Pubkey::from_str("SenderWallet1Here").unwrap(), true);
    tracked_wallets.insert(Pubkey::from_str("SenderWallet2Here").unwrap(), true);

    let (tx, mut rx) = mpsc::channel::<RpcTransactionLogs>(100);

    task::spawn(async move {
        listen_for_txs(rpc_url.to_string(), program_id, tx).await;
    });

    while let Some(log) = rx.recv().await {
        if let Some(sig) = log.signature {
            println!("Transaction found: {:?}", sig);
        }

        for account in &log.transaction.transaction.message.account_keys {
            if tracked_wallets.contains_key(account) {
                println!("Tracked wallet found as sender: {:?}", account);
            }
        }
    }
}

pub async fn run_listener_pubsub_service(
    program_id: Pubkey,
    ws_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        let client = PubsubClient::new(ws_url)
            .await
            .expect("pubsub client async");
        let (mut notifications, unsub) = client
            .logs_subscribe(
                RpcTransactionLogsFilter::Mentions(vec![
                    program_id.to_string(),
                ]),
                RpcTransactionLogsConfig {
                    commitment: Some(CommitmentConfig::processed()),
                },
            )
            .await
            .expect("subscribe to logs");
        while let Some(log) = notifications.next().await {
            // Handle logs here
        }
        unsub().await;
    })
    .await?;

    Ok(())
}

async fn listen_for_txs(ws_url: String, program_id: Pubkey, sender: mpsc::Sender<RpcTransactionLogs>) -> Result<(), Box<dyn std::error::Error>> {
    let config = RpcTransactionLogsConfig {
        commitment: Some(CommitmentConfig::confirmed()),
    };
    let filter = RpcTransactionLogsFilter::Mentions(vec![
        program_id.to_string()
    ]);

    let (subs, receiver) = PubsubClient::logs_subscribe(
        &ws_url,
        filter,
        config,
    )?;

    println!("listening to logs for {:?}", program_id);
    Ok((subs, receiver))
}

