use std::sync::Mutex;
use std::thread;
use std::time::Duration;


struct BankAccount {
    balance: Mutex<f64> ,
    account_numer: String,
}

impl BankAccount    {
    pub fn new(account_number: String, initial_balance: f64) -> Self {
        BankAccount {
            balance: Mutex::new(initial_balance),
            account_numer: account_number,
        }
    }

    pub fn deposit(&mut self,amount:f64) -> Self{

        let mut bal = self.balance.lock().unwrap();
        *bal += amount;
        println!("Deposited {} to account {}. New balance: {}", amount, self.account_numer, *bal);
        thread::sleep(Duration::from_millis(50));
        Self::new(self.account_numer.clone(),*bal)

    }

    pub fn transfer(&mut self,amount:f64,target:&mut BankAccount) ->  Result<(), String> {
       
        let (mut source_lock, mut target_lock ) = if self as *const _ < target as *const _ {
                   let s =  self.balance.lock().map_err(|e| e.to_string())?;
                   let t =  target.balance.lock().map_err(|e| e.to_string())?;
                   (s,t)
                } else {
                    let t = target.balance.lock().map_err(|e| e.to_string())?;
                    let s = self.balance.lock().map_err(|e| e.to_string())?;
                    (s,t)
                };
            if *source_lock < amount {
                println!("Insufficient funds in account {}. Transfer of {} failed.", self.account_numer, amount);
                return Err("Insufficient funds".to_string());
            }  
            *source_lock -= amount;
            *target_lock += amount;
            println!("Transferred {} from account {} to account {}. New balances: {} -> {}, {} -> {}", amount, self.account_numer, target.account_numer, self.account_numer, *source_lock, target.account_numer, *target_lock);
            thread::sleep(Duration::from_millis(50));
            Ok(())

        }
    pub fn get_balance(&self) -> f64{
        let bal = self.balance.lock().unwrap();
        *bal
    }

    

}

mod tests {
    use super::*;
    #[test]
    fn test_bank_account_operations() {
        let mut account1 = BankAccount::new("123".to_string(), 1000.0);
        let mut account2 = BankAccount::new("456".to_string(), 500.0);

        account1 = account1.deposit(200.0);
        assert_eq!(account1.get_balance(), 1200.0);

        account1.transfer(300.0, &mut account2).unwrap();
        assert_eq!(account1.get_balance(), 900.0);
        assert_eq!(account2.get_balance(), 800.0);

        let result = account1.transfer(1000.0, &mut account2);
        assert!(result.is_err());
        assert_eq!(account1.get_balance(), 900.0);
        assert_eq!(account2.get_balance(), 800.0);
    }
}
