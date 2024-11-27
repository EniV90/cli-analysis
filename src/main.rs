use solana_client::rpc_client::RpcClient;

fn main() {
    // Connect to the Solana Devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url);

    println!("Connecting to solana Devnet...");
    // Test connection
    match client.get_health() {
        Ok(_) => {
            println!("Connection successful! Solana Devnet is healthy.");
            // Get recent block height
            match client.get_slot() {
                Ok(slot) => println!("Current slot: {}", slot),
                Err(err) => eprintln!("Error fetching slot: {:?}", err),
            }
        }
        Err(err) => eprintln!("Failed to connect to Solana Devnet: {:?}", err),
    }
}