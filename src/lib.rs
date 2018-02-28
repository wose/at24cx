//! A platform agnostic driver to interface with the AT24C32/64 EEPROM.
//!
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![feature(unsize)]

extern crate byteorder;
extern crate embedded_hal as hal;

use core::cmp::min;
use core::marker::Unsize;
use core::mem;

use byteorder::{ByteOrder, BE};
use hal::blocking::i2c::{Write, WriteRead};

/// I2C address
///
/// The AT24C32/64 has three input pins `A2`, `A1` and `A0` to select the I2C
/// device address. The pin is also considered low if left floating.
#[derive(Copy, Clone)]
pub enum Address {
    /// Device address: `0x50`
    ///
    /// - `A2 = low`
    /// - `A1 = low`
    /// - `A0 = low`
    Addr0 = 0x50,
    /// Device address: `0x51`
    ///
    /// - `A2 = low`
    /// - `A1 = low`
    /// - `A0 = high`
    Addr1 = 0x51,
    /// Device address: `0x52`
    ///
    /// - `A2 = low`
    /// - `A1 = high`
    /// - `A0 = low`
    Addr2 = 0x52,
    /// Device address: `0x53`
    ///
    /// - `A2 = low`
    /// - `A1 = high`
    /// - `A0 = high`
    Addr3 = 0x53,
    /// Device address: `0x54`
    ///
    /// - `A2 = high`
    /// - `A1 = low`
    /// - `A0 = low`
    Addr4 = 0x54,
    /// Device address: `0x55`
    ///
    /// - `A2 = high`
    /// - `A1 = low`
    /// - `A0 = high`
    Addr5 = 0x55,
    /// Device address: `0x56`
    ///
    /// - `A2 = high`
    /// - `A1 = high`
    /// - `A0 = low`
    Addr6 = 0x56,
    /// Device address: `0x57`
    ///
    /// - `A2 = high`
    /// - `A1 = high`
    /// - `A0 = high`
    Addr7 = 0x57,
}

impl Address {
    fn bits(&self) -> u8 {
        *self as u8
    }
}

/// AT24Cx Driver
pub struct AT24Cx {
    address: Address,
}

impl AT24Cx {
    /// Creates a new driver with the given I2C address.
    pub fn new(address: Address) -> Self {
        AT24Cx { address: address }
    }

    /// Writes a single byte at the specified memory address and wait for the
    /// internally-timed write cycle to finish.
    pub fn write<I2C, E>(&self, i2c: &mut I2C, address: u16, byte: u8) -> Result<(), E>
    where
        I2C: Write<Error = E>,
    {
        let mut buffer = [0; 3];
        BE::write_u16(&mut buffer[0..2], address);
        buffer[2] = byte;
        i2c.write(self.address.bits(), &buffer)?;

        // wait until the write cycle is finished
        self.wait(i2c);
        Ok(())
    }

    /// Writes a single page starting at the given memory address and wait for
    /// the internally-timed write cycle to finish.
    pub fn write_page<I2C, E>(&self, i2c: &mut I2C, address: u16, data: &[u8]) -> Result<(), E>
    where
        I2C: Write<Error = E>,
    {
        // limit is the page size or we would overwrite data we jyst sent
        let len = min(data.len(), 32);

        // 2 address bytes + page size
        let mut buffer = [0; 34];
        {
            let (addr, dst) = buffer.split_at_mut(2);
            BE::write_u16(addr, address);
            dst[..len].clone_from_slice(&data[..len]);
        }

        i2c.write(self.address.bits(), &buffer[..data.len() + 2])?;

        // wait until the write cycle is finished
        self.wait(i2c);
        Ok(())
    }

    /// Reads an arbitrary amount of bytes starting at the given memory address.
    /// Reading will continue at the start of the memory if the address boundary is reached.
    pub fn read<B, I2C, E>(&self, i2c: &mut I2C, address: u16) -> Result<B, E>
    where
        B: Unsize<[u8]>,
        I2C: WriteRead<Error = E>,
    {
        let mut addr = [0; 2];
        BE::write_u16(&mut addr, address);

        let mut buffer: B = unsafe { mem::uninitialized() };
        {
            let slice: &mut [u8] = &mut buffer;
            i2c.write_read(self.address.bits(), &addr, slice)?;
        }

        Ok(buffer)
    }

    /// Waits for the internally-timed write cycle to finish.
    fn wait<I2C, E>(&self, i2c: &mut I2C)
    where
        I2C: Write<Error = E>,
    {
        while let Err(_) = i2c.write(self.address.bits(), &[]) {}
    }
}
