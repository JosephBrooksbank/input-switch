use std::error::Error;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::Trigger;
use rppal::system::DeviceInfo;
use std::time::{SystemTime};

const BUTTON_PIN: u8 = 17;

const SHORT_BUTTON_LIMIT: u16 = 250;
const NOISE_LIMITER: u8 = 1;
const DEBOUNCE_TIME: u8 = 20;
const DOUBLE_CLICK_TIMEOUT: u16 = 400;
// const INPUT_STATUS_PIN: u8 = 4;
// const OUTPUT_SWITCH_PIN: u8 = 17;
// const IR_TRANSMITTER_PIN: u8 = 23;

enum ButtonPressType {
    Short,
    Long
}

fn button_callback(_: Level) {
    println!("Button Pressed!");
}

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Switching input on board on a {}.",
        DeviceInfo::new()?.model()
    );
    let mut pin = Gpio::new()?.get(BUTTON_PIN)?.into_input_pullup();

    let res = pin.set_async_interrupt(Trigger::Disabled, button_callback);
    match res {
        Ok(_) => (),
        Err(e) => println!("Error: {e}"),
    }
    let mut button_down: SystemTime = SystemTime::now();
    let mut button_up: SystemTime = SystemTime::now();
    let mut sequence_start: SystemTime = SystemTime::now();
    let mut last_state: Level = Level::High;
    let mut button_presses: Vec<ButtonPressType> =  Vec::new();

    loop {
        if pin.is_low() {
            if last_state == Level::High {
                if sequence_start.elapsed()?.as_millis() > DOUBLE_CLICK_TIMEOUT.into() {
                    sequence_start = SystemTime::now();
                    button_presses = Vec::new();
                }
                button_down = SystemTime::now();
                last_state = Level::Low;
            }
        }
        if pin.is_high() {
            if last_state == Level::Low {
                if button_up.elapsed()?.as_millis() > DEBOUNCE_TIME.into() {
                    match button_down.elapsed() {
                        Ok(elapsed) => {
                            let milisecs = elapsed.as_millis();
                            if milisecs > NOISE_LIMITER.into() && milisecs < SHORT_BUTTON_LIMIT.into() {
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
        if button_presses.len() > 0 && sequence_start.elapsed()?.as_millis() > DOUBLE_CLICK_TIMEOUT.into() {
            let mut short_presses = 0;
            let mut long_presses = 0;

            for press in button_presses {
                match press {
                    ButtonPressType::Short => {short_presses+=1;}
                    ButtonPressType::Long => {long_presses+=1;}
                }
            }
            
            if short_presses == 1 {
                println!("Short click!");
            }
            if short_presses == 2 {
                println!("Double click!");
            }
            if long_presses > 0 {
                println!("Long press");
            }

            button_presses = Vec::new();
        }
    }
}
