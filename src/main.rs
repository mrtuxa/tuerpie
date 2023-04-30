use std::env;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use rppal::gpio::{Gpio, Level};
use rppal::gpio::Trigger;

// set your scripts for opening and closing the space here
static OPEN_SPACE: &str = "OpenSpace";
static CLOSE_SPACE: &str = "CloseSpace";
static DOOR_PIN: u8 = 13;
static RECHECK_DELAY: u64 = 30; // ins seconds

fn main() {
    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(DOOR_PIN).unwrap().into_input();

    // Set the trigger for the interrupt
    pin.set_interrupt(Trigger::Both).unwrap();

    loop {
        // reading space status from door
        let status = pin.poll_interrupt(true, None).unwrap().unwrap();
        pin.clear_interrupt().unwrap();
        let door_open = status == Level::Low;
        println!("GPIO pin {} triggered an interrupt", pin.pin());
        println!("Space is online = {}", door_open);

        println!("GPIO pin {} was triggered", DOOR_PIN);
        push_door_status(door_open);

        // make it more reliable and robust
        sleep(Duration::from_secs(RECHECK_DELAY));
        // check if the door has still the same status, if not push the new status to the api
        let new_door_open = pin.is_low();
        if door_open != new_door_open
        {
            push_door_status(new_door_open)
        }
        pin.set_interrupt(Trigger::Both).unwrap();
    }
}

fn push_door_status(open: bool) {
    let mut command = Command::new("sh");
    command
        .arg("-c")
        .arg(format!("SPACEAPI_URL={}", env::var("SPACEAPI_URL").unwrap()))
        .arg(format!("API_KEY={}", env::var("API_KEY").unwrap()));
    if open { command.arg(OPEN_SPACE); } else { command.arg(CLOSE_SPACE); }
    command.spawn()
        .expect("Failed to execute process. Push to api failed");
}
