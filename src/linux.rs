use linux_embedded_hal::{Delay, I2cdev};
use sdp8xx::Sdp8xx;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = 0x25;
    let mut sdp = Sdp8xx::new(dev, address, Delay);

    println!("Starting Sdp8xx tests.");

    let product_id = sdp.read_product_id().unwrap();
    println!("{:?}", product_id);
}
