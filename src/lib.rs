mod button;

#[cfg(target_arch = "arm")]
mod read;

#[cfg(target_arch = "arm")]
pub use read::button_events;

pub use crate::button::Button;

//Debug implementation for non-raspi
#[cfg( not(target_arch = "arm"))]
pub fn button_events() -> Result<Receiver<Button>, Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        loop {

        }
    });

    Ok(rx)
}