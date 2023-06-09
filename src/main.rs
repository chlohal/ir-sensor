use std::{error::Error, thread, time::{Instant, Duration}};

use rppal::{gpio::{Gpio, InputPin, Level}, system::DeviceInfo};

fn main() -> Result<(), Box<dyn Error>> {
    println!("device info: {}", DeviceInfo::new()?.model());

    let gpio = Gpio::new()?;

    let mut out_pin = gpio.get(24)?.into_output();
    let in_pin = gpio.get(23)?.into_input();

    out_pin.set_high();

    let base_state = true;

    loop {
        if let Some(message) = recieve_message(&in_pin) {
            println!("{:?}", message);
        }
    }

    Ok(())
}

#[derive(Debug)]
struct MessageFrame {
    device: u16,
    cmd: u16
}

fn recieve_message(pin: &InputPin) -> Option<MessageFrame> {
    //ensure we find a valid header component
    let high_header = time_period(pin, Level::High, 6000)?;
    let low_header = time_period(pin, Level::Low, 6000)?;

    if high_header.abs_diff(4500) > 500 || low_header.abs_diff(4500) > 500 {
        return None;
    }

    return Some(MessageFrame { 
        device: read_bytes(pin)?,
        cmd: read_bytes(pin)?
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

fn format_recieved_data(data: &Vec<(bool, u128)>) -> String {
    data
        .into_iter()
        .map(|x| format!("\x1b[{}m{}\x1b[0m", if x.0 { "31"  } else { "32"  },  x.1)  )
        .collect()
}

fn add_period_if_changed(last: bool, current: bool, time_in_current_period: u128, previous_end_time: u128,  data: &mut Vec<(bool, u128)>) -> u128 {
    if last != current {
        data.push((last, time_in_current_period - previous_end_time));

        return time_in_current_period;
    } else {
        return previous_end_time;
    }
}
