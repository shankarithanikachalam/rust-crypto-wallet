use std::sync::atomic::{AtomicU64, Ordering};

static WALLET_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Clone)]
struct Wallet {
    id: u64,
    balance: u64,
}

fn new_wallet(balance: u64) -> Wallet {
    let wallet_id = WALLET_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    Wallet {
        id: wallet_id,
        balance,
    }
}
fn check_balance(wallet: &Wallet) -> u64 {
    wallet.balance
}
fn send_money(wallet: &mut Wallet, amount: u64) -> (String, u64) {
    if wallet.balance >= amount {
        wallet.balance = wallet.balance - amount;
        (String::from("Sent Money"), wallet.balance)
    } else {
        (
            String::from("Insufficient balance in your wallet"),
            wallet.balance,
        )
    }
}
fn transfer_ownership(wallet: Wallet) -> Wallet {
    wallet
}
fn transfer_between(from: &mut Wallet, to: &mut Wallet, amount: u64) -> (u64, String) {
    if from.balance < amount {
        (
            from.balance,
            String::from("Insufficient balance in your wallet"),
        )
    } else {
        from.balance -= amount;
        to.balance += amount;
        (to.balance, String::from("Amount transferred"))
    }
}

fn get_wallet_info(wallet: &Wallet) -> String {
    format!(
        "\n Wallet ID: {} \n Wallet Balance: {}",
        wallet.id, wallet.balance
    )
}

fn batch_check(wallets: &[Wallet]) -> u64 {
    wallets.iter().map(|w| w.balance).sum()
}

fn clone_wallet(wallet: &Wallet) -> Wallet {
    let cloned_wallet_id = WALLET_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    Wallet {
        id: cloned_wallet_id,
        balance: wallet.balance,
    }
}
fn main() {
    let mut wallet_1 = new_wallet(15);
    println!("ID of ETH Wallet 1: {:?}", wallet_1.id);

    let wallet_1_balance = check_balance(&wallet_1);
    println!(
        "Balance of Wallet 1 without losing ownership {} ETH",
        wallet_1_balance
    );

    let amount_to_be_transferred = 10;
    let (status_msg, current_balance): (String, u64) =
        send_money(&mut wallet_1, amount_to_be_transferred);
    println!(
        "{} {} ETH and balance is {} ETH  ",
        status_msg, amount_to_be_transferred, current_balance,
    );

    let mut wallet_2 = transfer_ownership(wallet_1);
    println!(
        "The ownership of Wallet 1 has been transferred to Wallet 2, whose balance is {} ",
        wallet_2.balance
    );

    let mut wallet_3: Wallet = new_wallet(0);
    let mut wallet_4: Wallet = new_wallet(10);
    let mut wallet_5: Wallet = new_wallet(20);

    let cloned_wallet = clone_wallet(&wallet_5);

    let amount_transferred = 5;
    transfer_between(&mut wallet_2, &mut wallet_3, amount_transferred);
    println!(
        "The balance amount of {} ETH transferred from {} to {} ",
        amount_transferred, wallet_2.id, wallet_3.id
    );

    let wallet_info = get_wallet_info(&wallet_3);
    println!("The Wallet info: {}", wallet_info);

    let wallets_balance = vec![wallet_2, wallet_3, wallet_4.clone(), wallet_5.clone()];
    let balance_of_wallets = batch_check(&wallets_balance);
    println!("The balance of my current wallets: {} ", balance_of_wallets);

    // println!("The balance of Wallet 1 {}",wallet_1.balance);
    println!(
        "The balance of Cloned wallet (i.e., wallet 5 ) is {}",
        cloned_wallet.balance
    );

    // Wallet 5 is accessible because, the ownership is still retained, only its clone is used

    let amount_to_be_transferred = 24;
    let (current_balance, status_msg): (u64, String) =
        transfer_between(&mut wallet_4, &mut wallet_5, amount_to_be_transferred);

    println!(
        "Attempted to transfer {} ETH from wallet {} to wallet {}: {} | Remaining Balance: {} ETH",
        amount_to_be_transferred, wallet_4.id, wallet_5.id, status_msg, current_balance
    )
}
