mod bank;
mod wallet;
mod types;

use crate::bank::BankStorage;
use crate::wallet::WalletInfo;


fn main() {

    let mut db_storage = BankStorage::new();

    println!("Welcome to Bank!");

    loop {
        println!("What would you like to do?!");
        println!("available cmds:");
        println!(r#"
            1 [q] for quit
            2 [ll] for list current wallets
            3 [add/rem:username] for add/rem user info
            4 [username:add/sub:value] add/sub balance for a specific user"#
        );

        let cmds = read_commands();
        if cmds.len() == 0 { continue;}

        let cmd_size = cmds.len();
        match  cmd_size {
            3 => {
                let (user, operation, value_str) = (&cmds[0], &cmds[1], &cmds[2]);
                handle_balance_operation(&mut db_storage, user, operation, value_str)
            }
            2 => {
                let (operation, username) = (&cmds[0], &cmds[1]);
                handle_user_operation(&mut db_storage, username, operation)
            }
            1 => {
                let operation = cmds[0].trim();
                match operation {
                    "q" => { println!("quitting"); break; }
                    "ll" => { handle_list_user_wallets(&mut db_storage); }
                    _ => { println!("Invalid cmd!"); }
                }
            }
            _ => {
                println!("Invalid command!");
            }
        }

    }
}

fn read_commands() -> Vec<String> {
    let mut cmd = String::new();
    match std::io::stdin().read_line(&mut cmd) {
        Ok(_) => println!("value read from stdin"),
        Err(error) => {
            println!("error: {}", error);
            return Vec::new();
        },
    }
    let cmds = cmd.trim().split(":");
    cmds.map(|s| s.to_string()).collect()
}

fn handle_list_user_wallets(storage: &mut impl types::Bank) {
    println!("listing users\n");
    for wallet in storage.list_current_wallets() {
        println!("current wallet: \n");
        wallet.display_wallet();
    }
    println!("\n");
}

fn handle_user_operation(storage: &mut impl types::Bank, user: &str, operation: &str){
    match  operation {
        "add" => {
            let new_wallet = WalletInfo::new(user.to_string(), 0);
            match storage.add_wallet(new_wallet) {
                Ok(_) => println!("Wallet for user {user} created successfully"),
                Err(e) => println!("Failed creating wallet: {}", e)
            };
        }

        "rem" => {
            match storage.remove_wallet(user) {
                Ok(_) => println!("Wallet for user {user} removed successfully"),
                Err(e) => println!("Failed removing wallet: {}", e),
            }
        }

        _ => {
            println!("Invalid operation!");
        }
    }


}
fn handle_balance_operation(storage: &mut impl types::Bank, user: &str, operation: &str, value_str: &str) {
    if operation != "add" && operation != "sub" {
        println!("Invalid cmd.!");
        return;
    }

    let Ok(value) = value_str.parse::<u128>() else {
        println!("Invalid value.");
        return;
    };

    let wallet = match storage.get_wallet(&user) {
        Ok(wallet) => wallet,
        Err(e) => { return println!("Error. {e}");}
    };

    match operation {
        "add" => {
            match wallet.add_balance(value) {
                Ok(_) => {println!("Balance added!");},
                Err(e) => { return println!("Error. {e}");}
            }
        }
        "sub" => {
            match wallet.sub_balance(value) {
                Ok(_) => {println!("Balance subtracted!");},
                Err(e) => { return println!("Error. {e}");}
            }
        }
        _ => unreachable!()
    }
}
