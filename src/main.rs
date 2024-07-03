mod tests;

struct BankAccount {
    account_number: u32,
    balance: f64,
}

impl BankAccount {
    // Function to create a new bank account
    fn new(account_number: u32, initial_balance: f64) -> BankAccount {
        todo!()
    }

    // Function to get account balance
    fn get_balance(&self) -> f64 {
        self.balance
    }

    // Function to deposit money
    fn deposit(self, amount: f64) -> BankAccount {
        todo!()
    }

    // Function to withdraw money
    fn withdraw(self, amount: f64) -> BankAccount {
        todo!()
    }

    // Function to transfer money from one account to another
    fn transfer(self, to_account: BankAccount, amount: f64) -> (BankAccount, BankAccount) {
        todo!()
    }
}

fn main() {
    let account1 = BankAccount::new(123456, 1000.0);
    let account2 = BankAccount::new(654321, 500.0);

    // Print initial balances
    println!(
        "Initial Balance of Account 1: ${:.2}",
        account1.get_balance()
    );
    println!(
        "Initial Balance of Account 2: ${:.2}",
        account2.get_balance()
    );

    // Deposit money into account1
    let account1 = account1.deposit(500.0);
    println!(
        "Balance of Account 1 after deposit: ${:.2}",
        account1.get_balance()
    );

    // Withdraw money from account1
    let account1 = account1.withdraw(300.0);
    println!(
        "Balance of Account 1 after withdrawal: ${:.2}",
        account1.get_balance()
    );

    // Transfer money from account1 to account2
    let (account1, account2) = account1.transfer(account2, 200.0);

    // Print balances after transfer
    println!(
        "Balance of Account 1 after transfer: ${:.2}",
        account1.get_balance()
    );
    println!(
        "Balance of Account 2 after transfer: ${:.2}",
        account2.get_balance()
    );
}
