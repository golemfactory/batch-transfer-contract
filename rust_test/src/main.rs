use anyhow;
use env_logger;
use log;
use std::ops::{Add, Mul};
use tokio::fs;
use web3;
use web3::{
    contract::{Contract, Options},
    types::U256,
};

/*async fn run() -> anyhow::Result<()> {
    let provider = web3::transports::Http::new("https://bor.golem.network")?;

    let web3 = web3::Web3::new(provider);
    let accounts = web3.eth().accounts().await;

    log::info!("Accounts: {:?}", accounts);
    Ok(())
}*/

use std::str::FromStr;

use secp256k1::SecretKey;

use web3::types::{Address, H160, TransactionParameters};

/// Below generates and signs a transaction offline, before transmitting it to a public node (eg Infura)
/// For sending a transaction to a local node that stores private keys (eg Ganache) see transaction_private
async fn run() -> anyhow::Result<()> {
    // Sign up at infura > choose the desired network (eg Rinkeby) > copy the endpoint url into the below
    // If you need test ether use a faucet, eg https://faucet.rinkeby.io/
    //let transport = web3::transports::Http::new("https://rpc-mumbai.maticvigil.com/v1/fd04db1066cae0f44d3461ae6d6a7cbbdd46e4a5")?;
    //let transport = web3::transports::Http::new("http://1.geth.testnet.golem.network:55555")?;
    let transport = web3::transports::Http::new("https://bor.golem.network")?;
    let web3 = web3::Web3::new(transport);

    let universal_transaction_options = Options {
        gas_price: Some(U256::from(36_100_000_000u64)),
        gas: Some(U256::from(1_000_000)),
        access_list: None,
        condition: None,
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
        transaction_type: None,
        value: None,
        nonce: None,
    };

    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let to = Address::from_str("0xFcbcA3BBD5De05331eD2FdC38E3B4a4084C7DF3D")?;

    // Insert the 32-byte private key in hex format (do NOT prefix with 0x)
    let private_key = fs::read_to_string("../private_key.txt").await?;
    let prvk = SecretKey::from_str(&private_key)?;

    //let contract_address = Address::from_str("0x32d22cb5303a18a6f613ed77307e791273d8a472")?;

    //let contract_address = Address::from_str("0x612Cca90C672A49Ad25Ca5C61a9fC56F50F70000")?;
    //let contract_address = Address::from_str("0x6655930f910C4659f31D83763De30C4cCB9105AD")?;
    //let contract_address = Address::from_str("0x22a5eC48b703F3C42A2e8fB45C3C03f08653F8Bf")?;

    //let contract_address = Address::from_str("0x489983D573D0A9F1fC461324CA194B313DE18e08")?;

    let contract_address = Address::from_str("0x398c9D361119f5d4ea3C6996340e8BDE4DD5C4Fe")?;
    let contract_address = Address::from_str("0xC803c478e0e28b311B704578724bBB6C416e6D6a")?;
    let contract_address = Address::from_str("0x2A0426c36303e279E4b60a18fD6F2260aF88e089")?;

    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../../MultiTransferERC20.abi.json"),
    )?;

    let glm_address: Address = contract
        .query("GLM", (), None, Options::default(), None)
        .await?;

    let glm_contract = Contract::from_json(
        web3.eth(),
        glm_address,
        include_bytes!("../../ERC20.abi.json"),
    )?;

    //let owner = Address::from_str("0x89Ef977db64A2597bA57E3eb4b717D3bAAeBaeC3")?;
    let owner = Address::from_str("0xBbd57FCC7a388f88016fdae4b0BdbAED95DAC49F")?;

    let allowance: U256 = glm_contract
        .query(
            "allowance",
            (owner, contract_address),
            None,
            Options::default(),
            None,
        )
        .await?;

    log::debug!("Address allowance: {}", allowance);

    let total_transfer_glm = U256::from(100);

    if allowance < total_transfer_glm {
        log::debug!("Insufficient allowance, calling approve");
        let treceipt = glm_contract
            .signed_call_with_confirmations(
                "approve",
                (contract_address, U256::max_value()),
                universal_transaction_options.clone(),
                1,
                &prvk,
            )
            .await?;
        log::debug!("approved {:?}", treceipt);
    }

    let receivers:Vec<H160> = vec![
        /* Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20000")?,
            Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20001")?,
                      Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20002")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20003")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20004")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20005")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20006")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20007")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20008")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20009")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20010")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20011")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20012")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20013")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20014")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20015")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20016")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20017")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20018")?,
         Address::from_str("0x0b220b82f3ea3b7f6d9a1d8ab58930c064a20019")?,*/
    ];

    let amounts:Vec<U256> = vec![
        /*U256::from(44778),
          U256::from(1238),
                U256::from(3),
        U256::from(4),
        U256::from(5),
        U256::from(6),
        U256::from(7),
        U256::from(8),
        U256::from(9),
        U256::from(10),
        U256::from(1),
        U256::from(2),
        U256::from(3),
        U256::from(4),
        U256::from(5),
        U256::from(6),
        U256::from(7),
        U256::from(8),
        U256::from(9),
        U256::from(10),*/
    ];

    let amountSum = amounts.iter().fold(U256::from(0), |sum, e| sum + e);
    let packing = true;
    if packing {
        let packed: Vec<[u8; 32]> = receivers
            .iter()
            .zip(amounts.iter())
            .map(|(&receiver, &amount)| {
                let mut packet2 = [0u8; 32];
                amount.to_big_endian(&mut packet2[..]);
                packet2[..20].copy_from_slice(&receiver[..20]);
                packet2
            })
            .collect();

        let gas_estimation = contract
            .signed_call(
                //"golemTransferDirectPacked",
                //   "golemTransferIndirectPacked",
                "golemTransferIndirectPacked",
                (packed, amountSum),
                //        owner,
                universal_transaction_options.clone(),
                //          1,
                &prvk,
            )
            .await?;
        log::debug!("Gas estimation: {:?}", gas_estimation);
    } else {
        let gas_estimation = contract
            .signed_call(
                "golemTransferIndirect",
                (receivers, amounts),
                //        owner,
                universal_transaction_options.clone(),
                //          1,
                &prvk,
            )
            .await?;

        log::debug!("Gas estimation: {:?}", gas_estimation);
    }

    // Build the tx object
    /*let tx_object = TransactionParameters {
        to: Some(to),
        value: U256::from(10), //0.1 eth
        ..Default::default()
    };*/

    // Sign the tx (can be done offline)
    //    let signed = web3.accounts().sign_transaction(tx_object, &prvk).await?;

    // Send the tx to infura
    //let result = web3.eth().send_raw_transaction(signed.raw_transaction).await?;

    //llog::info!("Tx succeeded with hash: {}", result);

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    run()
        .await
        .unwrap_or_else(|err| log::error!("Error during execution: {:?}", err));
}
