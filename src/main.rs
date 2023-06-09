use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::OutputPin;
use rppal::system::DeviceInfo;
use std::time::SystemTime;

const POLLING_DELAY_MS: u8 = 1;
const BUTTON_PIN: u8 = 17;

const SHORT_BUTTON_LIMIT: u16 = 250;
const NOISE_LIMITER: u8 = 1;
const DEBOUNCE_TIME: u8 = 20;
const DOUBLE_CLICK_TIMEOUT: u16 = 400;

const OUTPUT_SWITCH_PIN_1: u8 = 18;
const OUTPUT_SWITCH_PIN_2: u8 = 27;
// const INPUT_STATUS_PIN: u8 = 27;
// const INPUT_STATUS_PIN: u8 = 4;
// const IR_TRANSMITTER_PIN: u8 = 23;

enum ButtonPressType {
    Short,
    Long,
}

// #[derive(Debug)]
// enum OutputType {
//     Desktop,
//     Laptop,
// }

// fn button_callback(level: Level) {
//     if level == Level::High {
//         println!("Input is now on laptop");
//     } else if level == Level::Low {
//         println!("Input is now on desktop");
//     }
// }

fn switch_input(pin: &mut OutputPin) {
    pin.set_low();
    thread::sleep(Duration::from_millis(1000));
    pin.set_high();
}

// fn get_current_output(pin: &mut InputPin) -> OutputType {
//     if pin.is_high() {
//         OutputType::Laptop
//     } else {
//         OutputType::Desktop
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Switching input on board on a {}.",
        DeviceInfo::new()?.model()
    );
    let button_pin = Gpio::new()?.get(BUTTON_PIN)?.into_input_pullup();
    let mut switch_pin_kbm = Gpio::new()?.get(OUTPUT_SWITCH_PIN_1)?.into_output_low();
    let mut switch_pin_audio = Gpio::new()?.get(OUTPUT_SWITCH_PIN_2)?.into_output_low();
    // let mut status_pin = Gpio::new()?.get(INPUT_STATUS_PIN)?.into_input();

    // status_pin.set_async_interrupt(Trigger::Both, button_callback);

    let mut button_down: SystemTime = SystemTime::now();
    let mut button_up: SystemTime = SystemTime::now();
    let mut sequence_start: SystemTime = SystemTime::now();
    let mut last_state: Level = Level::High;
    let mut button_presses: Vec<ButtonPressType> = Vec::new();

    loop {
        if button_pin.is_low() {
            if last_state == Level::High {
                if sequence_start.elapsed()?.as_millis() > DOUBLE_CLICK_TIMEOUT.into() {
                    sequence_start = SystemTime::now();
                    button_presses = Vec::new();
                }
                button_down = SystemTime::now();
                last_state = Level::Low;
            }
        }
        if button_pin.is_high() {
            if last_state == Level::Low {
                if button_up.elapsed()?.as_millis() > DEBOUNCE_TIME.into() {
                    match button_down.elapsed() {
                        Ok(elapsed) => {
                            let milisecs = elapsed.as_millis();
                            if milisecs > NOISE_LIMITER.into()
                                && milisecs < SHORT_BUTTON_LIMIT.into()
                            {
                                button_presses.push(ButtonPressType::Short)
                            } else if milisecs > SHORT_BUTTON_LIMIT.into() {
                                button_presses.push(ButtonPressType::Long);
                            }
                        }
                        Err(e) => {
                            println!("Error: {e:?}");
                        }
                    }
                    button_up = SystemTime::now();
                }
            }
            last_state = Level::High;
        }
        if button_presses.len() > 0
            && sequence_start.elapsed()?.as_millis() > DOUBLE_CLICK_TIMEOUT.into()
        {
            let mut short_presses = 0;
            let mut long_presses = 0;

            for press in button_presses {
                match press {
                    ButtonPressType::Short => {
                        short_presses += 1;
                    }
                    ButtonPressType::Long => {
                        long_presses += 1;
                    }
                }
            }

            if short_presses == 1 {
                switch_input(&mut switch_pin_kbm);
            } else if short_presses == 2 {
                switch_input(&mut switch_pin_audio);
            } else if long_presses > 0 {
                switch_input(&mut switch_pin_kbm);
                switch_input(&mut switch_pin_audio);
            }

            button_presses = Vec::new();
        }

        thread::sleep(Duration::from_millis(POLLING_DELAY_MS.into()));
    }
}
