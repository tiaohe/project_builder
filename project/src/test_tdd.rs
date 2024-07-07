#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(balance: f64) -> Self {
        Self { balance }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err(String::from("Insufficient balance"))
        }
    }

    pub fn check_balance(&self) -> f64 {
        self.balance
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    //检测 账户余额初始化
    #[test]
    fn test_new_account() {
        let mut account = BankAccount::new(100.0);
        account.deposit(100.0);
        assert_eq!(account.check_balance(), 200.0);
    }

    // 检测消费过后余额正确
    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(50.0).expect("余额不足");
        assert_eq!(account.check_balance(), 50.0);
    }

    // 检测消费过度后余额不足 报错
    #[test]
    fn test_withdraw_failed() {
        let mut account = BankAccount::new(100.0);
        let result = account.withdraw(150.0);
        assert!(result.is_err());
    }

}


