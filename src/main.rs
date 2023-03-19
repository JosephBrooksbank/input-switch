use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const BUTTON_PIN: u8 = 27;
const INPUT_STATUS_PIN: u8 = 4;
const OUTPUT_SWITCH_PIN: u8 = 17;
const IR_TRANSMITTER_PIN: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Switching input on board on a {}.",
        DeviceInfo::new()?.model()
    );

    let mut pin = Gpio::new()?.get(OUTPUT_SWITCH_PIN)?.into_output();
    loop {
        pin.set_low();
        thread::sleep(Duration::from_millis(1000));
        pin.set_high();
    }

    Ok(())
}
