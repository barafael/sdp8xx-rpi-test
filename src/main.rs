use linux_embedded_hal::{Delay, I2cdev};
use sdp8xx::Sdp8xx;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = 0x25;
    let mut sdp = Sdp8xx::new(dev, address, Delay);

    println!("Starting Sdp8xx tests.");

    let product_id = sdp.read_product_id().unwrap();
    println!("{:?}", product_id);

    println!("Taking 10 triggered samples");
    for _ in 0..=10 {
        if let Ok(m) = sdp.read_sample_triggered() {
            println!("{:?}", m);
        } else {
            println!("Error!");
        }
    }

    let mut sdp_sampling = match sdp.start_sampling_differential_pressure(true) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            loop {}
        }
    };
    for _ in 0..=50 {
        let result = sdp_sampling.read_continuous_sample();
        match result {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("Error while getting result: {:?}", e),
        }
    }
    let mut idle_sensor = sdp_sampling.stop_sampling().unwrap();
    loop {
        if let Ok(m) = idle_sensor.read_sample_triggered() {
            println!("{:?}", m);
        } else {
            println!("Error!");
        }
    }
}
