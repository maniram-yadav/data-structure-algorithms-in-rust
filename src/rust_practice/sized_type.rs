https://practice.course.rs/newtype-sized.html

use std::fmt;

/* Define the Wrapper type */
struct Wrapper(Vec<String>);

// Display is an external trait
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    // Vec is an external type, so you cannot implement Display trait on Vec type
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

=================
/* Make it workd */
struct Meters(u32);

impl Meters{
    fn pow(&self,p:u32) -> i32 {
        
        (self.0.pow(p))as i32
    }
}

fn main() {
    let i: u32 = 2;
    assert_eq!(i.pow(2), 4);

    let n = Meters(i);
    // The `pow` method is defined on `u32` type, we can't directly call it 
    assert_eq!(n.pow(2), 4);
}


============
/* Make it work */
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

// An age verification function that checks age in years, must be given a value of type Years.
fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
}

============
use std::ops::Add;
use std::fmt::{self, format};

struct Meters(u32);
impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There are still {} meters left", self.0)
    }
}

impl Add for Meters {
    type Output = Self;

    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0)
    }
}
fn main() {
    let d = calculate_distance(Meters(10), Meters(20));
    assert_eq!(format!("{}",d), "There are still 30 meters left");
    println!("{}",d);
}

/* Implement calculate_distance  */
fn calculate_distance(m1:Meters,m2:Meters) -> Meters{
    let m = m1+m2;
    m
    
}


===============
type Thunk = Box<dyn Fn() + Send + 'static>;


fn takes_long_type(f: Thunk) {
    f();
}

fn returns_long_type() -> Thunk {
     Box::new(|| println!("hi"))
}

fn main(){
    takes_long_type(returns_long_type());
}


=================
/* Make it work with const generics */
fn my_function<const N:usize>() -> [u32; N] {
    [123; N]
}

fn main() {
    let arr = my_function::<3>();
    println!("{:?}",arr);
    let arr = my_function::<5>();
    println!("{:?}",arr);

    
}

===============
/* Make it work with slice references */
fn main() {
    let s: &str = "Hello there!";

    let arr: &[u8] = &[1, 2, 3];
    println!("Len : {}",arr.len());
    println!("Len : {}",s.len());
    
}
==============
/* Make it work in two ways */
use std::fmt::Display;
fn foobar(thing: impl Display) {
    println!("{}",thing);
}    

fn main() {
      foobar(1);
}
=========