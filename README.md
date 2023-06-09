This project is a client for recieving IR signals from a Philips SRP4419V/27 Universal Remote on a Raspberry Pi.

For it to work correctly, there must be some kind of IR detector. I used a scavenged one from an LED controller, which I wire-stripped & tied onto the GPIO. 
There must be an output to the sensor on pin 24, which will always be 3.3v. There must be an input from the sensor on pin 23. To be honest, I'm not really sure how the sensor that I have works (i.e. exact voltages) -- I just know it works like this.

All the buttons on the remote work, but you need to use the right profile. This project uses the factory reset profile (which can be accessed by holding down the "SETUP" button for 5 seconds, then pressing the MUTE button, then pressing the number "0")-- I couldn't find any other profile which supported every button.

It's important to know that the "ENTER" button on the numpad and the d-pad "OK" button have the same code, which I've referred to as `Button::Enter`.

This is a very rough project so far. Calling the `button_events()` function will give you a channel of `Button` events. There is no timestamp associated with buttons; they are simply issued as soon as the transmission is recieved.

On non-arm platforms, the channel will never yield anything.