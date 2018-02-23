//! A platform agnostic driver to interface with the AT24C32/64 EEPROM.
//!
//!

#![no_std]

extern crate embedded_hal as hal;

use core::marker::PhantomData;
use hal::blocking::i2c::{Write, WriteRead};

// we'll add support for the other 7 addresses later
pub const ADDRESS: u8 = 0x57;

/// AT24Cx Driver
pub struct AT24Cx<I2C> {
    _i2c: PhantomData<I2C>,
}

impl<I2C, E> AT24Cx<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Creates a new driver.
    pub fn new() -> Self {
        AT24Cx { _i2c: PhantomData }
    }

    /// Writes a single byte at the specified address.
    pub fn write(&self, i2c: &mut I2C, address: u16, byte: u8) -> Result<(), E> {
        let msb = (address >> 8) as u8;
        let lsb = (address & 0xFF) as u8;
        i2c.write(ADDRESS, &[msb, lsb, byte])
    }

    /// Reads a single byte from the specified address.
    pub fn read(&self, i2c: &mut I2C, address: u16) -> Result<u8, E> {
        let msb = (address >> 8) as u8;
        let lsb = (address & 0xFF) as u8;
        let mut buffer = [0];
        i2c.write_read(ADDRESS, &[msb, lsb], &mut buffer)?;
        Ok(buffer[0])
    }
}
