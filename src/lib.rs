mod button;

#[cfg(target_arch = "arm")]
mod read;

#[cfg(target_arch = "arm")]
pub use read::button_events;

pub use crate::button::Button;

//Debug implementation for non-raspi
#[cfg( not(target_arch = "arm"))]
pub fn button_events() -> Result<std::sync::mpsc::Receiver<Button>, Box<dyn std::error::Error>> {
    let (tx, rx) = std::sync::mpsc::channel();
    
    std::thread::spawn(move || {
        loop {
            tx.send(Button::Home).unwrap();
        }
    });

    Ok(rx)
}