mod configuration;
mod presets;
mod shared;

use configuration::{Configuration, ConfigurationError};
use openrgb::{data::Color, OpenRGB};
use presets::all_presets;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    path::Path,
    process::exit,
    sync::Arc,
    time::Duration,
};

const APP_NAME: &'static str = env!("CARGO_CRATE_NAME");

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

        settings = Configuration::deserialize(toml::Deserializer::new(&buffer))?;
    } else {
        let mut config_file = File::create(config_path)?;

        let default_config = Configuration::default();

        let mut default_config_string = String::new();
        default_config.serialize(toml::Serializer::new(&mut default_config_string))?;

        config_file.write_all(default_config_string.as_bytes())?;

        log::info!("Created config file at {}", config_path.to_str().unwrap());

        settings = default_config;
    }

    return Ok(settings);
}

/*
    App flow:
    Load config -> Connect -> Select controller ->
    -> Select mode -> Display
*/

type OpenRGBClient = OpenRGB<tokio::net::TcpStream>;

async fn run_preset(client: Arc<OpenRGBClient>, controller_id: u32, preset_id: usize) {
    let controller = client.get_controller(controller_id).await.unwrap();
    let led_count = controller.leds.len();

    let mut modes = all_presets();

    log::info!(
        "Running {} leds on controller {} (ID {}).",
        led_count,
        controller.name,
        controller_id,
    );

    let mut screen = vec![Color::new(0, 0, 0); led_count];

    log::info!(
        "Selected mode '{}' ({}) for controller {} (ID {})",
        modes[preset_id].name(),
        preset_id,
        controller.name,
        controller_id
    );

    modes[preset_id].init();
    loop {
        modes[preset_id].update(&mut screen);

        client
            .update_leds(controller_id as u32, screen.clone())
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_nanos(10_000_000)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let settings = setup_config()?;

    log::info!("Loaded configuration successfully.");
    
    if settings.version != configuration::CURRENT_FORMAT_VERSION { 
        log::warn!("Configuration is outdated (v{} > v{}). Some things might break!",
            configuration::CURRENT_FORMAT_VERSION, settings.version
        )
    }
    
    if settings.controller_configs.len() == 0 {
        log::warn!("Controller configuration is empty! Refer to README.md for configuration guides.")
    }

    log::info!("Available presets:");
    for (idx, preset) in all_presets().iter().enumerate() {
        log::info!("- ({}) {}", idx, preset.name())
    }
    
    let client = Arc::new(OpenRGB::connect().await?); 
 
    ctrlc::set_handler(|| {
        log::info!("Stopping...");
        exit(0);
    })
    .expect("Error setting CTRL-C as handler");
    
    let mut tasks = Vec::new();
    
    for (_controller_name, controller_config) in settings.controller_configs {
        let client = client.clone();
        
        tasks.push(tokio::spawn(async move {
            run_preset(client, controller_config.controller_id as u32, controller_config.selected_mode).await
        }));
    }
    
    for task in tasks {
        task.await?;
    }
    
    Ok(())
}
