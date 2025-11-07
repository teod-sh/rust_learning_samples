use crate::types::Wallet;

pub trait Bank {
    fn add_wallet<T: Wallet + 'static>(&mut self, wallet: T) -> Result<(), String>;
    fn remove_wallet(&mut self, username: &str) -> Result<(), String>;
    fn list_current_wallets(&self) -> impl Iterator<Item = &dyn Wallet> + '_;
    fn get_wallet(&mut self, username: &str) -> Result<&mut dyn Wallet, String>;
}