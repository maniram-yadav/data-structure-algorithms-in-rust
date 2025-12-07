macro_rules! pow {
    (squared $n:expr) => {
        $n.pow(2)
    };
    (cubed $n:expr) => {
        $n.pow(3)
    };
}

fn main() {
    assert_eq!(pow!(squared 2_i32), 4);
    assert_eq!(pow!(cubed 2_i32), 8);
}a