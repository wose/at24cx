//! A platform agnostic driver to interface with the AT24C32/64 EEPROM.
//!
//!

#![no_std]
#![feature(unsize)]

extern crate byteorder;
extern crate embedded_hal as hal;

use core::cmp::min;
use core::marker::Unsize;
use core::mem;

use byteorder::{ByteOrder, BE};
use hal::blocking::i2c::{Write, WriteRead};

// we'll add support for the other 7 addresses later
pub const ADDRESS: u8 = 0x57;

/// AT24Cx Driver
pub struct AT24Cx;

impl AT24Cx {
    /// Creates a new driver.
    pub fn new() -> Self {
        AT24Cx {}
    }

    /// Writes a single byte at the specified address.
    pub fn write<I2C, E>(&self, i2c: &mut I2C, address: u16, byte: u8) -> Result<(), E>
    where
        I2C: Write<Error = E> + WriteRead<Error = E>,
    {
        let mut buffer = [0; 3];
        BE::write_u16(&mut buffer[0..2], address);
        buffer[2] = byte;
        i2c.write(ADDRESS, &buffer)
    }

    /// Writes a single page starting at the given address.
    pub fn write_page<I2C, E>(&self, i2c: &mut I2C, address: u16, data: &[u8]) -> Result<(), E>
    where
        I2C: Write<Error = E> + WriteRead<Error = E>,
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

        i2c.write(ADDRESS, &buffer[..data.len() + 2])
    }

    /// Reads an arbitrary amount of bytes starting at the given memory address.
    /// Reading will continue at the start of the memory if the address boundary is reached.
    pub fn read<B, I2C, E>(&self, i2c: &mut I2C, address: u16) -> Result<B, E>
    where
        B: Unsize<[u8]>,
        I2C: Write<Error = E> + WriteRead<Error = E>,
    {
        let mut addr = [0; 2];
        BE::write_u16(&mut addr, address);

        let mut buffer: B = unsafe { mem::uninitialized() };
        {
            let slice: &mut [u8] = &mut buffer;
            i2c.write_read(ADDRESS, &addr, slice)?;
        }

        Ok(buffer)
    }
}
