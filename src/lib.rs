#![no_std]

use core::{error::Error, fmt::Display};
#[derive(Debug,Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
pub enum NoError{

}
impl Display for NoError{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self{

        }
    }
}
impl Error for NoError{

}