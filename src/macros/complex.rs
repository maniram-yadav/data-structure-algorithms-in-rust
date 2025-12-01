macro_rules! rpn {
    { @inner_op stack [$r:expr, $l:expr $(, $stack:expr)*]; $op:tt $($tt:tt)* } => {
        rpn! { @inner stack [$l $op $r $(, $stack)*]; $($tt)* }
    };
    { @inner stack [$res:expr]; } => { $res };
    { @inner stack $stack:tt; + $($tt:tt)* } => {
        rpn!{ @inner_op stack $stack; + $($tt)* }
    };
    { @inner stack $stack:tt; - $($tt:tt)* } => {
        rpn!{ @inner_op stack $stack; - $($tt)* }
    };
    { @inner stack $stack:tt; * $($tt:tt)* } => {
        rpn!{ @inner_op stack $stack; * $($tt)* }
    };
    { @inner stack $stack:tt; / $($tt:tt)* } => {
        rpn!{ @inner_op stack $stack; / $($tt)* }
    };
    { @inner stack [$($stack:expr),*]; $num:tt $($tt:tt)* } => {
        rpn!{ @inner stack [$num $(, $stack)*]; $($tt)* }
    };
    { $($tt:tt)* } => {
       rpn!{ @inner stack [ ]; $($tt)* }
    };
}

fn main() {

assert_eq! { rpn! {3 4 +}, 7};
assert_eq! { rpn! {3 4 + 5 +}, 12};
assert_eq! { rpn! {3 4 5 + *}, 27};
assert_eq! { rpn! {15 7 1 1 + - / 3 * 2 1 1 + + -}, 5};
}