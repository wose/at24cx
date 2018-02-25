extern crate at24cx;
extern crate linux_embedded_hal as hal;

use at24cx::AT24Cx;
use hal::I2cdev;
use std::thread;
use std::time::Duration;

fn main() {
    let mut dev = I2cdev::new("/dev/i2c-1").unwrap();
    let eeprom = AT24Cx::new();
    eeprom.write_page(&mut dev, 32, &[0xEE; 32]).unwrap();
    thread::sleep(Duration::from_millis(10));

    let mem: [u8; 4096] = eeprom.read(&mut dev, 0x0000).unwrap();
    for page in mem.chunks(32) {
        for byte in page {
            print!("{:X} ", byte);
        }
        println!();
    }
}
