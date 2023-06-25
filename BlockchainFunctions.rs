use substrate_subxt::{balances, sp_runtime::AccountId32}; // to interact with Polkadot node and query token balance
use std::str::FromStr;
use sp_core::H256;
use frame_system::offchain::SendTransactionTypes;
use frame_system::offchain::{AppCrypto, CreateSignedTransaction};
use pallet_vesting::{self as vesting, VestingInfo};


// Connects to Polkadot node, converts account string to AccountId32 and queries the token balance at the specified time
async fn check_token_balance(
    account: &str,
    time: u64,
    nodeUrl: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the Polkadot node
    let transport = substrate_subxt::Transport::new(nodeUrl).await?;
    let client = substrate_subxt::ClientBuilder::<substrate_subxt::DefaultNodeRuntime>::new()
        .set_url(nodeUrl)
        .build()
        .await?;

    // Convert the account string to an AccountId32
    let account_id = AccountId32::from_str(account)?;

    // Query the token balance
    let balance_query = balances::BalancesQuery::<substrate_subxt::DefaultNodeRuntime>::new()
        .account_id(account_id)
        .at(time);
    let balance = balance_query.get(&client).await?;
    
    return (time, balance);
}

#[tokio::token_balance]
async fn token_balance(
    account: &str, // Specify the user's account
    time: i32, // Desired time
) -> Result<(), Box<dyn std::error::Error>> {
    return check_token_balance(account, time).await?;
}

fn lock_tokens<T: vesting::Config>(
    account: &T::AccountId,
    amount: T::Balance,
    duration: T::BlockNumber,
) {
    let vesting_info = VestingInfo {
        locked: amount,
        per_block: amount,
        starting_block: T::BlockNumber::from(0), // Replace with the appropriate starting block
        ending_block: duration, // Replace with the appropriate ending block based on the duration
    };

    vesting::Module::<T>::vested_transfer(account.clone(), vesting_info);
}

fn lock_token_in_account(
    account: AccountId,
    amount: Balance,
    duration: BlockNumber,
) {
    lock_tokens::<Runtime>(&account, amount, duration);
}


async fn send_xcmp_message(
    target_parachain_id: u32,
    zk_proof: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare the XCMP message payload
    let xcmp_payload = (zk_proof,);

    // Send the XCMP message to the target parachain
    let result = frame_system::Pallet::<DefaultNodeRuntime>::send_xcmp_message(
        target_parachain_id.into(),
        xcmp_payload.encode(),
    )
    .await;

    match result {
        Ok(()) => println!("XCMP message sent successfully"),
        Err(e) => println!("Failed to send XCMP message: {:?}", e),
    }

    Ok(())
}

#[tokio::send_zk_proof]
async fn send_zk_proof(
    target_parachain_id: i32, // Specify the target parachain ID and the zk proof
    zk_proof: vec!,
) -> Result<(), Box<dyn std::error::Error>> {
    send_xcmp_message(target_parachain_id, zk_proof).await?;
    Ok(())
}
