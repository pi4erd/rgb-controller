mod presets;
mod shared;

use openrgb::{data::Color, OpenRGB};
use std::{error::Error, process::exit, time::Duration};
use presets::PixelFunction;

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

    const CONTROLLER_ID: u32 = 2;

    let controller = client.get_controller(CONTROLLER_ID).await?;
    let led_count = controller.leds.len();

    println!(
        "Running {} leds on controller {}.",
        led_count, controller.name
    );

    let mut screen = vec![Color::new(0, 0, 0); led_count];

    let mut modes = load_presets! [
        DefaultBlinks,
    ];

    println!("Available modes:");
    for (idx, mode) in modes.iter().enumerate() {
        println!("- [{}] {}", idx, mode)
    }

    let selected: usize = 0;

    println!(
        "Selected mode '{}' with ID {}",
        modes[selected].name(),
        selected
    );

    modes[selected].init();

    loop {
        modes[selected].update(&mut screen);

        client
            .update_leds(CONTROLLER_ID, screen.clone())
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_nanos(10_000_000)).await;
    }
}
