extern crate at24cx;
extern crate linux_embedded_hal as hal;

use at24cx::AT24Cx;
use hal::I2cdev;
use std::thread;
use std::time::Duration;

fn main() {
    let mut dev = I2cdev::new("/dev/i2c-1").unwrap();
    let eeprom = AT24Cx::new();

    eeprom.write(&mut dev, 0x0042, 42).unwrap();

    // wait 10ms for the write to finish or the eeprom will NAK the next write or read request
    thread::sleep(Duration::from_millis(10));

    println!(
        "The answer to the ultimate question of life, the universe and everything is {}.",
        eeprom.read(&mut dev, 0x0042).unwrap()
    );
}
