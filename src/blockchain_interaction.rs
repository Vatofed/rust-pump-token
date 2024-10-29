use solana_client::{client_error::ClientError, rpc_client::RpcClient};
use solana_sdk::{signature::{Keypair, Signer}, transaction::Transaction, pubkey::Pubkey, system_instruction};
use std::str::FromStr;

const SOLANA_CLUSTER_URL: &str = "https://api.mainnet-beta.solana.com";

pub struct BlockchainInteraction {
    client: RpcClient,
    payer: Keypair,
}

impl BlockchainInteraction {
    pub fn new(payer: Keypair) -> Self {
        Self {
            client: RpcClient::new(SOLANA_CLUSTER_URL),
            payer,
        }
    }

    pub fn create_token(&self, token_name: &str, total_supply: u64) -> Result<Pubkey, ClientError> {
        let token_account = Keypair::generate(&mut rand::thread_rng());

        let create_account_ix = system_instruction::create_account(
            &self.payer.pubkey(),
            &token_account.pubkey(),
            self.client.get_minimum_balance_for_rent_exemption(0)?,
            0,
            &Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
        );

        let mut transaction = Transaction::new_with_payer(
            &[create_account_ix],
            Some(&self.payer.pubkey()),
        );

        let recent_blockhash = self.client.get_latest_blockhash()?;
        transaction.sign(&[&self.payer, &token_account], recent_blockhash);

        self.client.send_and_confirm_transaction(&transaction)?;

        Ok(token_account.pubkey())
    }
}
