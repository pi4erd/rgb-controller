mod configuration;
mod presets;
mod shared;

use configuration::{Configuration, ConfigurationError};
use openrgb::{data::Color, OpenRGB};
use presets::PixelFunction;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    path::Path,
    process::exit,
    time::Duration,
};

macro_rules! load_presets {
    ($($name:ident $(,)?)+) => {
        {
            let mut tmp_vec: Vec<Box<dyn PixelFunction>> = Vec::new();
            $(tmp_vec.push(Box::new(presets::$name::default()));)*
            tmp_vec
        }
    };
}

const APP_NAME: &'static str = env!("CARGO_CRATE_NAME");

fn default_config() -> &'static str {
    "controller_id = 2
selected_mode = 0
"
}

fn setup_config() -> Result<Configuration, Box<dyn Error>> {
    let config_path_str: &str = &format!(
        "{}/{}/config.toml",
        dirs::config_dir()
            .ok_or(ConfigurationError("Config directory unavailable"))?
            .to_str()
            .unwrap(),
        APP_NAME
    );

    let settings: Configuration;

    let config_path = Path::new(config_path_str);

    log::trace!("Config file at {}", config_path_str);

    let config_dir = config_path.parent().unwrap();

    log::trace!("Config dir at {}", config_dir.to_str().unwrap());

    if !config_dir.exists() {
        std::fs::create_dir(config_dir)?;
    }

    if config_path.exists() {
        let mut config_file = File::open(config_path)?;

        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;

        settings = Configuration::deserialize(&buffer)?;
    } else {
        let mut config_file = File::create(config_path)?;
        config_file.write_all(default_config().as_bytes())?;

        log::info!("Created config file at {}", config_path.to_str().unwrap());

        settings = Configuration::deserialize(default_config())?;
    }

    return Ok(settings);
}

/*
    App flow:
    Load config -> Connect -> Select controller ->
    -> Select mode -> Display
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let settings = setup_config()?;
    
    log::info!("Loaded configuration successfully.");

    let client = OpenRGB::connect().await?;

    ctrlc::set_handler(|| {
        log::info!("Stopping...");
        exit(0);
    })
    .expect("Error setting CTRL-C as handler");

    let controller = client.get_controller(settings.controller_id as u32).await?;
    let led_count = controller.leds.len();

    log::info!(
        "Running {} leds on controller {} (ID {}).",
        led_count,
        controller.name,
        settings.controller_id
    );

    let mut screen = vec![Color::new(0, 0, 0); led_count];

    let mut modes = load_presets![DefaultBlinks,];

    log::info!("Available modes:");
    for (idx, mode) in modes.iter().enumerate() {
        log::info!("- [{}] {}", idx, mode)
    }

    log::info!(
        "Selected mode '{}' with ID {}",
        modes[settings.selected_mode].name(),
        settings.selected_mode
    );

    modes[settings.selected_mode].init();

    loop {
        modes[settings.selected_mode].update(&mut screen);

        client
            .update_leds(settings.controller_id as u32, screen.clone())
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_nanos(10_000_000)).await;
    }
}
