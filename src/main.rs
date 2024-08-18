mod config;
mod presets;
mod shared;

use config::Configuration;
use openrgb::{data::Color, OpenRGB};
use presets::PixelFunction;
use std::{error::Error, process::exit, time::Duration};

macro_rules! load_presets {
    ($($name:ident $(,)?)+) => {
        {
            let mut tmp_vec: Vec<Box<dyn PixelFunction>> = Vec::new();
            $(tmp_vec.push(Box::new(presets::$name::default()));)*
            tmp_vec
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = OpenRGB::connect().await?;

    ctrlc::set_handler(|| {
        println!("Stopping...");
        exit(0);
    })
    .expect("Error setting CTRL-C as handler");

    // let controller_id = force_input_int("Enter controlled id (e.g. 0): ") as u32;

    let config = Configuration {
        controller_id: 2,
        selected_mode: 0,
    };

    let controller = client.get_controller(config.controller_id as u32).await?;
    let led_count = controller.leds.len();

    println!(
        "Running {} leds on controller {}.",
        led_count, controller.name
    );

    let mut screen = vec![Color::new(0, 0, 0); led_count];

    let mut modes = load_presets![DefaultBlinks,];

    println!("Available modes:");
    for (idx, mode) in modes.iter().enumerate() {
        println!("- [{}] {}", idx, mode)
    }

    println!(
        "Selected mode '{}' with ID {}",
        modes[config.selected_mode].name(),
        config.selected_mode
    );

    modes[config.selected_mode].init();

    loop {
        modes[config.selected_mode].update(&mut screen);

        client
            .update_leds(config.controller_id as u32, screen.clone())
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_nanos(10_000_000)).await;
    }
}
