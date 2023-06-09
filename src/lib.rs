mod button;

#[cfg(target_arch = "arm")]
mod read;

#[cfg(target_arch = "arm")]
pub use read::button_events;

pub use crate::button::Button;

//Debug implementation for non-raspi
#[cfg( not(target_arch = "arm"))]
pub fn button_events() -> Result<impl Iterator<Item = Button>, Box<dyn std::error::Error>> {
    Ok((vec![Button::ArrowDown]).into_iter())
}