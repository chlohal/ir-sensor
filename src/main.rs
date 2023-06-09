use std::{error::Error, thread, time::{Instant, Duration}};

use rppal::{gpio::Gpio, system::DeviceInfo};

fn main() -> Result<(), Box<dyn Error>> {
    println!("device info: {}", DeviceInfo::new()?.model());

    let gpio = Gpio::new()?;

    let mut out_pin = gpio.get(24)?.into_output();
    let in_pin = gpio.get(23)?.into_input();

    out_pin.set_high();

    let base_state = true;

    let mut last_state: bool = base_state;
    let mut last_changed: u128 = 0;
    let mut first_time: Option<Instant> = None;
    let mut recieved: Vec<(bool, u128)> = Vec::new();

    loop {
        let state = in_pin.is_high();

        if first_time.is_none() && state != base_state {
            first_time = Some(Instant::now());
            recieved = Vec::new();
            last_state = base_state;
            last_changed = 0;
        }

        if let Some(start) = first_time {
            let time_since_transmission = Instant::now().duration_since(start).as_micros();
            let time_since_last_change = time_since_transmission - last_changed;

            if time_since_last_change > 1_000_000 {
                first_time = None;
                print!("{}\n\n", format_recieved_data(&recieved));
                continue;
            }

            last_changed = add_period_if_changed(last_state, state, time_since_transmission, last_changed, &mut recieved);

            last_state = state;
        }
    }

    Ok(())
}

struct MessageFrame {
    device: u8,
    cmd: u8
}

fn recieve_message(pin: &InputPin) -> Option<MessageFrame> {
    //ensure we find a valid header component
    let high_header = time_period(pin, Level::High, 6000)?
    let low_header = time_period(pin, Level::Low, 6000)?

    if high_header.abs_diff(4500) > 500 || low_header.abs_diff(4500) > 500 {
        return None;
    }

    let device = read_byte(pin);
    let device_inv = read_byte(pin);

    let cmd = read_byte(pin);
    let cmd_inv = read_byte(pin);

    return MessageFrame { device, cmd }; 
}

fn read_byte(pin: &InputPin) -> Option<u8> {
    let mut byte: u8  = 0;

    for bit in 0..7 {
        byte |= (read_bit(pin)? << bit);
    }

    return byte;
}


///Binary values are encoded as follows:
/// Logical 1: a 0.5622 ms burst followed by a 1.687ms low period
/// Logical 0: a 0.5622 ms burst followed by a 0.5622 ms low period
///digikey.com/en/maker/blogs/2021/understanding-the-basics-of-infrared-communications
fn read_bit(pin: &InputPin) -> Option<u8> {
    let burst = time_period(pin, Level::High, 700);
    if burst.abs_diff(562) > 100 { return None; }

    let low_period = time_period(pin, Level::Low, 2000);

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
