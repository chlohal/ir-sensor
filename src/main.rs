fn main() -> Result<(), Box<dyn std::error::Error>> {
    let events = ir_sensor::button_events()?;
    loop {
        println!("{:?}", events.recv()?);
    }
}