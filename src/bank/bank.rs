use std::collections::HashMap;

pub use crate::types::Bank;
pub use crate::types::Wallet;

pub struct BankStorage{
    db: HashMap<String, Box<dyn Wallet>>,
}

impl BankStorage {
    pub fn new() -> impl Bank {
        Self {
            db: HashMap::new(),
        }
    }
}

impl Bank for BankStorage{

    fn add_wallet<T: Wallet + 'static>(&mut self, new_wallet: T) -> Result<(), String> {
        let wallet_box = Box::new(new_wallet);
        match self.db.insert(wallet_box.get_username().to_string(), wallet_box) {
            Some(before) => {
                println!("User have previously added wallet which now got replaced by a new one");
                println!("Previous Wallet: ");
                before.display_wallet();
                Ok(()) // may be changed to error
            },
            None => Ok(()),
        }
    }

    fn remove_wallet(&mut self, username: &str) -> Result<(), String>{
        self.db.remove(username)
            .map(|_| ())
            .ok_or_else(|| format!("Wallet \"{}\" does not exists", username))
    }

    fn list_current_wallets(&self) -> impl Iterator<Item = &dyn Wallet> + '_{
        self.db.values().map(|wallet| wallet.as_ref())
    }

    fn get_wallet(&mut self, username: &str) -> Result<&mut dyn Wallet, String> {
        match self.db.get_mut(username) {
            Some(wallet) => Ok(wallet.as_mut()),
            None => Err(format!("User {} not found", username)),
        }
    }
}
