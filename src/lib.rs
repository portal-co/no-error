#![no_std]

use core::{error::Error, fmt::Display};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NoError {}
impl Display for NoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {}
    }
}
impl Error for NoError {}
#[cfg(feature = "embedded-io")]
impl embedded_io::Error for NoError {
    fn kind(&self) -> embedded_io::ErrorKind {
        match *self {}
    }
}
#[cfg(feature = "embedded-hal")]
const _: () = {
    impl embedded_hal::digital::Error for NoError {
        fn kind(&self) -> embedded_hal::digital::ErrorKind {
            match *self {}
        }
    }
    impl embedded_hal::i2c::Error for NoError {
        fn kind(&self) -> embedded_hal::i2c::ErrorKind {
            match *self {}
        }
    }
    impl embedded_hal::pwm::Error for NoError {
        fn kind(&self) -> embedded_hal::pwm::ErrorKind {
            match *self {}
        }
    }
    impl embedded_hal::spi::Error for NoError {
        fn kind(&self) -> embedded_hal::spi::ErrorKind {
            match *self {}
        }
    }
};
