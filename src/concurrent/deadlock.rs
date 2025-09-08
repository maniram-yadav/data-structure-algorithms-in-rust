
    //   #[test]
    // fn concurrent_transfers() {

    //     let account_a = Arc::new(BankAccount::new("123".to_string(),1000.0));
    //     let account_b = Arc::new(BankAccount::new("456".to_string(),500.0));

    //     let handles: Vec<_> = (0..10)
    //         .map(|i| {
    //             let  a = Arc::clone(&account_a);
    //             let b = Arc::clone(&account_b);

    //             thread::spawn(move || {
    //                 thread::sleep(Duration::from_millis(i * 10));
    //                 if i % 2 == 0 {
    //                     a.transfer( 50.0,&mut Arc::clone(&account_b)).unwrap()
                    
    //                 } else {
    //                     b.transfer(25.0,&mut a).unwrap()
    //                 }
    //             })
    //         })
    //         .collect();

    //     for handle in handles {
    //         handle.join().unwrap();
    //     }

    //     println!(
    //         "Final balances - A: {:.2}, B: {:.2}",
    //         *account_a.balance.lock().unwrap(),
    //         *account_b.balance.lock().unwrap()
    //     );
    // }

    