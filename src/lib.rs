#![allow(non_camel_case_types)]
use std::ops::Shl;
use std::fmt::{Display, Formatter, Error as FmtError};

pub struct cout;
pub struct endl;

impl Display for endl {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        writeln!(f, "")
    }
}

impl<T: Display> Shl<T> for cout {
    type Output = Self;
    fn shl(self, rhs: T) -> Self {
        println!("{}", rhs);
        Self
    }
}
#[macro_export]
macro_rules! i {(int main()$m:block)=>{fn main() {let _:i32=(|| {$m})();}}}