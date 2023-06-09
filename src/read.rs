
use std::{error::Error, time::Instant};

use rppal::gpio::{Gpio, InputPin, Level};

use crate::button::Button;

const REMOTE_DEVICE: u16 = 1799;

pub fn button_events() -> Result<impl Iterator<Item = Button>, Box<dyn Error>> {
    let in_pin = setup_pins()?;

    Ok(InfraredButtons(in_pin)) 
}

struct InfraredButtons(InputPin);

impl Iterator for InfraredButtons {
    type Item = Button;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(MessageFrame {cmd, device: REMOTE_DEVICE}) = recieve_message(&self.0) {
                if let Ok(button) = Button::try_from(cmd) {
                    return Some(button);
                }
            }
        }
    }
}

fn setup_pins() -> Result<InputPin, Box<dyn Error>> {
    let gpio = Gpio::new()?;

    let mut out_pin = gpio.get(24)?.into_output();
    let in_pin = gpio.get(23)?.into_input();

    out_pin.set_high();

    Ok(in_pin)
}

#[derive(Debug)]
struct MessageFrame {
    device: u16,
    cmd: u8
}

fn recieve_message(pin: &InputPin) -> Option<MessageFrame> {
    //ensure we find a valid header component
    let high_header = time_period(pin, Level::High, 6000)?;
    let low_header = time_period(pin, Level::Low, 6000)?;

    if high_header.abs_diff(4500) > 500 || low_header.abs_diff(4500) > 500 {
        return None;
    }

    let device = read_bytes(pin)?;

    let cmd = read_bytes(pin)?;
    let cmd_inv: u8 = read_bytes(pin)?;

    //the xor of the inverse will have all bits filled!
    //if it's not, then fail
    if cmd_inv ^ cmd != 255 { return None; }

    return Some(MessageFrame { 
        device, cmd
    });
}

fn read_bytes<T: Default + std::ops::BitOrAssign<T> + std::ops::Shl<usize, Output = T> + std::convert::From<u8>>(pin: &InputPin) -> Option<T> {
    let mut byte: T = Default::default();

    for bit in 0..(std::mem::size_of::<T>()*8) {
        byte |= <u8 as Into<T>>::into(read_bit(pin)?) << bit;
    }

    return Some(byte);
}


///Binary values are encoded as follows:
/// Logical 1: a 0.5622 ms burst followed by a 1.687ms low period
/// Logical 0: a 0.5622 ms burst followed by a 0.5622 ms low period
///digikey.com/en/maker/blogs/2021/understanding-the-basics-of-infrared-communications
fn read_bit(pin: &InputPin) -> Option<u8> {
    let burst = time_period(pin, Level::High, 700)?;
    if burst.abs_diff(562) > 100 { return None; }

    let low_period = time_period(pin, Level::Low, 2000)?;

    if low_period.abs_diff(1687) < 100 { return Some(1); }
    if low_period.abs_diff(562) < 100 { return Some(0); }

    return None; 
}

fn time_period(pin: &InputPin, level: Level, timeout: u128) -> Option<u128> {
    let start = Instant::now();

    loop {
        let duration = Instant::now().duration_since(start).as_micros();
        if pin.read() == level {
            return Some(duration);
        }
        if duration > timeout {
            return None;
        }
    }
}
