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

    println!("Going to sleep!");
    let sleeping = sdp.go_to_sleep().unwrap();

    println!("Sleeping.");
    std::thread::sleep(std::time::Duration::from_millis(500));

    let sdp = sleeping.wake_up().unwrap();
    println!("Woken up!");

    let mut sdp_sampling = match sdp.start_sampling_differential_pressure(true) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            loop {}
        }
    };
    std::thread::sleep(std::time::Duration::from_millis(100));
    println!("Starting to take all the samples");

    for _ in 0..=50 {
        let result = sdp_sampling.read_continuous_sample();
        match result {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("Error while getting result: {:?}", e),
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
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
