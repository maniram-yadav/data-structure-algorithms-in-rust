macro_rules! adder {
    () => { 0 };
    ($($n:expr),*) => {
        {
            let mut sum = 0;
            $(
                sum += $n;
            )*
            sum
        }
    };
}

fn main() {
    assert_eq!(adder!(1, 2, 3, 4), 10);
    assert_eq!(adder!(1), 1);
    assert_eq!(adder!(), 0);
}