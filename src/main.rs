use std::env;
use std::thread::sleep;
use std::time::Duration;

use futures::executor::block_on;
use rppal::gpio::{Gpio, InputPin, Level};
use spaceapi_dezentrale_client::Client;

// set your scripts for opening and closing the space here
static DOOR_PIN: u8 = 27;
// delays in seconds
static RECHECK_DELAY: u64 = 5;
static ANTI_BOUNCE_DELAY: u64 = 1;

fn main() {
    // setup
    let gpio = Gpio::new().unwrap();
    let pin = gpio.get(DOOR_PIN).unwrap().into_input();

    let spaceapi_client = spaceapi_dezentrale_client::ClientBuilder::new()
        .api_key(&env::var("API_KEY").unwrap())
        .base_url(&env::var("SPACEAPI_URL").unwrap())
        .build()
        .unwrap();

    // get initial door status
    let mut door_status_old = check_door(&pin);
    block_on(push_door_status(&spaceapi_client, door_status_old));

    loop {
        // maybe not the best solution but the pi isn't doing anything else
        println!("Waiting {} secs to read the door status", RECHECK_DELAY);
        sleep(Duration::from_secs(RECHECK_DELAY));

        // read new status
        let door_status_new = check_door(&pin);
        println!("Read {}", door_status_new);

        // if the new status isn't the old one
        if door_status_old != door_status_new {
            // wait for the switch to stop bouncing around
            println!("Waiting {} secs for recheck", ANTI_BOUNCE_DELAY);
            sleep(Duration::from_secs(ANTI_BOUNCE_DELAY));
            // the new read status is still the same after a minute then push it to the api
            if door_status_new == check_door(&pin)
            {
                println!("Check passed, applying new status");
                println!("Pushing space status: Open = {}", door_status_new);
                block_on(push_door_status(&spaceapi_client, door_status_new));
                println!("Saving space status: {}", door_status_new);
                door_status_old = door_status_new;
            } else {
                println!("Check wasn't successfully")
            }
        }
    }
}

fn check_door(pin: &InputPin) -> bool {
    // reading space status from door
    pin.read() == Level::Low
}

async fn push_door_status(spaceapi: &Client, open: bool) {
    if open { spaceapi.open().await.unwrap() } else { spaceapi.close().await.unwrap() }
}
