macro_rules! calculator {
    {add $($add:expr),*; mul $($mul:expr),*} => {
       {
            let mut sum = 0;
            let mut mul = 1;
            $(
                sum += $add;
            )*
            $(
                mul *= $mul;
            )*
            (sum, mul)
        }
    }
}

fn main() {
    assert_eq!(calculator!(add 1, 2, 3; mul 1, 2, 3), (6, 6));
}