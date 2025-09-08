use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::sync::Arc;

struct BankAccount {
    balance: Mutex<f64>,
    account_numer: String,
}

impl BankAccount {
    pub fn new(account_number: String, initial_balance: f64) -> Self {
        BankAccount {
            balance: Mutex::new(initial_balance),
            account_numer: account_number,
        }
    }

    pub fn deposit(&mut self, amount: f64) -> Self {
        let mut bal = self.balance.lock().unwrap();
        *bal += amount;
        println!(
            "Deposited {} to account {}. New balance: {}",
            amount, self.account_numer, *bal
        );
        thread::sleep(Duration::from_millis(50));
        Self::new(self.account_numer.clone(), *bal)
    }

    pub fn transfer(&mut self, amount: f64, target: &mut BankAccount) -> Result<(), String> {
        let (mut source_lock, mut target_lock) = if self as *const _ < target as *const _ {
            let s = self.balance.lock().map_err(|e| e.to_string())?;
            let t = target.balance.lock().map_err(|e| e.to_string())?;
            (s, t)
        } else {
            let t = target.balance.lock().map_err(|e| e.to_string())?;
            let s = self.balance.lock().map_err(|e| e.to_string())?;
            (s, t)
        };
        if *source_lock < amount {
            println!(
                "Insufficient funds in account {}. Transfer of {} failed.",
                self.account_numer, amount
            );
            return Err("Insufficient funds".to_string());
        }
        *source_lock -= amount;
        *target_lock += amount;
        println!(
            "Transferred {} from account {} to account {}. New balances: {} -> {}, {} -> {}",
            amount,
            self.account_numer,
            target.account_numer,
            self.account_numer,
            *source_lock,
            target.account_numer,
            *target_lock
        );
        thread::sleep(Duration::from_millis(50));
        Ok(())
    }
    pub fn get_balance(&self) -> f64 {
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

    #[test]
    fn concurrent_transfers() {
    // Create thread-safe wrappers
    let account_a = Arc::new(ThreadSafeAccount::new(BankAccount::new("1".to_string(), 1000.0)));
    let account_b = Arc::new(ThreadSafeAccount::new(BankAccount::new("2".to_string(), 500.0)));
    let account_c = Arc::new(ThreadSafeAccount::new(BankAccount::new("3".to_string(), 750.0)));

    let mut handles = vec![];

    // Transfer from A to B
    for i in 0..5 {
        let a = Arc::clone(&account_a);
        let b = Arc::clone(&account_b);
        
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 20));
            match a.transfer(&b, 50.0) {
                Ok(_) => println!("Thread {}: A->B transfer successful", i),
                Err(e) => println!("Thread {}: A->B transfer failed: {}", i, e),
            }
        }));
    }

    // Transfer from B to C
    for i in 5..10 {
        let b = Arc::clone(&account_b);
        let c = Arc::clone(&account_c);
        
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 20));
            match b.transfer(&c, 25.0) {
                Ok(_) => println!("Thread {}: B->C transfer successful", i),
                Err(e) => println!("Thread {}: B->C transfer failed: {}", i, e),
            }
        }));
    }

    // Transfer from C to A
    for i in 10..15 {
        let c = Arc::clone(&account_c);
        let a = Arc::clone(&account_a);
        
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 20));
            match c.transfer(&a, 10.0) {
                Ok(_) => println!("Thread {}: C->A transfer successful", i),
                Err(e) => println!("Thread {}: C->A transfer failed: {}", i, e),
            }
        }));
    }

    // Complex multi-account transfers
    for i in 15..20 {
        let a = Arc::clone(&account_a);
        let b = Arc::clone(&account_b);
        let c = Arc::clone(&account_c);
        
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 20));
            
            // Chain transfers: A->B then B->C
            if i % 2 == 0 {
                if let Ok(_) = a.transfer(&b, 30.0) {
                    thread::sleep(Duration::from_millis(10));
                    match b.transfer(&c, 15.0) {
                        Ok(_) => println!("Thread {}: A->B->C chain successful", i),
                        Err(e) => println!("Thread {}: B->C chain failed: {}", i, e),
                    }
                }
            } else {
                // Direct transfer C->A
                match c.transfer(&a, 20.0) {
                    Ok(_) => println!("Thread {}: C->A direct successful", i),
                    Err(e) => println!("Thread {}: C->A direct failed: {}", i, e),
                }
            }
        }));
    }

    // Wait for all transfers to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify final balances
    println!("\nFinal Balances:");
    println!("Account {}: {:.2}", account_a.account.lock().unwrap().account_number, account_a.get_balance());
    println!("Account {}: {:.2}", account_b.account.lock().unwrap().account_number, account_b.get_balance());
    println!("Account {}: {:.2}", account_c.account.lock().unwrap().account_number, account_c.get_balance());

    // Verify conservation of money
    let total_initial = 1000.0 + 500.0 + 750.0;
    let total_final = account_a.get_balance() + account_b.get_balance() + account_c.get_balance();
    
    println!("Total money conservation: {:.2} -> {:.2} ({})", 
        total_initial, total_final, 
        if (total_initial - total_final).abs() < 0.01 { "PASS" } else { "FAIL" });
}

// Additional test cases
fn test_insufficient_funds() {
    println!("\n=== Testing Insufficient Funds ===");
    
    let account_a = ThreadSafeAccount::new(BankAccount::new("1".to_string(), 100.0));
    let account_b = ThreadSafeAccount::new(BankAccount::new("2".to_string(), 50.0));
    
    // Try to transfer more than available
    match account_a.transfer(&account_b, 200.0) {
        Ok(_) => println!("Unexpected: Transfer should have failed"),
        Err(e) => println!("Expected failure: {}", e),
    }
    
    println!("Account 1 balance: {:.2}", account_a.get_balance());
    println!("Account 2 balance: {:.2}", account_b.get_balance());
}


    
}





struct ThreadSafeAccount {
    account: Mutex<BankAccount>,
}

impl ThreadSafeAccount {
    fn new(account: BankAccount) -> Self {
        Self {
            account: Mutex::new(account),
        }
    }

    fn transfer(&self, target: &ThreadSafeAccount, amount: f64) -> Result<(), String> {
        let mut source = self.account.lock().unwrap();
        let mut target = target.account.lock().unwrap();
        source.transfer(amount,&mut *target            )
    }

    fn get_balance(&self) -> f64 {
        self.account.lock().unwrap().get_balance()
    }
}