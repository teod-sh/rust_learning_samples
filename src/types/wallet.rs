
pub trait Wallet {
    fn display_wallet(&self);
    fn get_username(&self) -> &str;
    fn add_balance(&mut self, balance: u128) -> Result<(), String>;
    fn sub_balance(&mut self, balance: u128) -> Result<(), String>;
}