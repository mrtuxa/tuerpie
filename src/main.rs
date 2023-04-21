use rppal::gpio::Trigger;
use rppal::gpio::{Gpio, Level};
use std::process::Command;

fn main() {
    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(13).unwrap().into_input();

    // Set the trigger for the interrupt
    pin.set_interrupt(Trigger::Both).unwrap();

    loop {
        // Wait for an interrupt to occur
        if let Some(Level::High) = pin.poll_interrupt(true, None).unwrap() {
            println!("GPIO pin 13 was triggered");
            Command::new("sh")
            	.arg("-c")
            	.arg("SPACEAPI_URL=http://10.131.185.175:8000 API_KEY=not-very-secure ./spaceapi-dezentrale-client open")
            	.output()
            	.expect("failed to execute process");
            // Do something when the interrupt is triggered
        } else if let Some(Level::Low) = pin.poll_interrupt(true, None).unwrap() {
            println!("GPIO pin 13 was triggered");
            Command::new("sh")
            	.arg("-c")
            	.arg("SPACEAPI_URL=10.131.185.175:8000 API_KEY=not-very-secure ./spaceapi-dezentrale-client close")
            	.output()
            	.expect("failed to execute process");
        }
    }
}
