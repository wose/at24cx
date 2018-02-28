extern crate at24cx;
extern crate linux_embedded_hal as hal;

use at24cx::{AT24Cx, Address};
use hal::I2cdev;

fn main() {
    let mut dev = I2cdev::new("/dev/i2c-1").unwrap();
    let eeprom = AT24Cx::new(Address::Addr7);

    // uncomment to fill page 1 with 0xEE
    // eeprom.write_page(&mut dev, 32, &[0xEE; 32]).unwrap();

    // read the entire memory and print it pagewise
    let mem: [u8; 4096] = eeprom.read(&mut dev, 0x0000).unwrap();
    for page in mem.chunks(32) {
        for byte in page {
            print!("{:X} ", byte);
        }
        println!();
    }
}
