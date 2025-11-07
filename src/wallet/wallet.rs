use crate::types::Wallet;

pub struct WalletInfo {
    username: String,
    balance: u128,
}

impl WalletInfo {
    pub fn new(username: String, balance: u128) -> WalletInfo {
        Self { username, balance }
    }
}

impl Wallet for WalletInfo {

    fn display_wallet(&self) {
        println!("Wallet {}: {}", self.username, self.balance);
    }

    fn get_username(&self) -> &str {
        &self.username
    }

    fn add_balance(&mut self, balance: u128) -> Result<(), String> {
        self.balance += balance;
        Ok(())
    }

    fn sub_balance(&mut self, balance: u128) -> Result<(), String> {
        if balance > self.balance {
            Err("insufficient balance".to_string())
        } else {
            self.balance -= balance;
            Ok(())
        }
    }
}