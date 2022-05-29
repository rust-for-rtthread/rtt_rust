//! Implementation of [`embedded-hal`] serial traits
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal

use nb;
use rtt_rs2::api::*;

pub struct RttSerial {
    dev: APIRawCDevive,
}

impl RttSerial {
    pub fn open(name: &str) -> Option<RttSerial> {
        let dev = cdev_find(name)?;
        return if !is_eok(cdev_open(dev, *APIRawCDevOflag::zero().readwrite())) {
            None
        } else {
            Some(RttSerial { dev })
        };
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RttSerialError(RttCResult);

impl embedded_hal::serial::Error for RttSerialError {
    fn kind(&self) -> embedded_hal::serial::ErrorKind {
        use embedded_hal::serial::ErrorKind::*;
        Other
    }
}

impl embedded_hal::serial::ErrorType for RttSerial {
    type Error = RttSerialError;
}

impl embedded_hal::serial::nb::Read<u8> for RttSerial {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let mut buffer = [0; 1];
        let bytes_read = cdev_read(self.dev, -1, &mut buffer);
        if bytes_read == 1 {
            Ok(buffer[0])
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl embedded_hal::serial::nb::Write<u8> for RttSerial {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        cdev_write(self.dev, 0, &[word; 1]);
        Ok(())
    }
    
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(())
    }
}
