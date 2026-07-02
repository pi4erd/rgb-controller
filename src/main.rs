mod configuration;
mod errors;
mod presets;
mod shared;

use configuration::{Configuration, ConfigurationError};
use openrgb::{OpenRGB, data::Color};
use presets::all_presets;
use tokio::sync::Mutex;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{Read, Write},
    process::exit,
    sync::Arc,
    time::Duration,
};

use crate::errors::RunError;

const APP_NAME: &'static str = env!("CARGO_CRATE_NAME");

struct AppState {
    running: bool,
    client: OpenRGBClient,
}

fn setup_config() -> Result<Configuration, Box<dyn Error>> {
    // TODO: Configuration pathwalking,
    // e.g. /etc/... -> ~/.config/... -> ~/.rgb-controller/config.toml
    let config_dir = dirs::config_dir()
        .ok_or(ConfigurationError("Config directory unavailable"))?;
    if !config_dir.exists() {
        std::fs::create_dir(&config_dir)?;
    }

    let config_path = config_dir.join(format!("{APP_NAME}/config.toml"));

    let settings: Configuration;

    log::trace!("Config file at {}", config_path.to_string_lossy());

    if config_path.exists() {
        let mut config_file = File::open(config_path)?;

        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;

        settings = toml::from_str(&buffer)?;
    } else {
        let mut config_file = File::create(&config_path)?;

        let default_config = Configuration::default();

        let default_config_string = toml::to_string(&default_config)?;

        config_file.write_all(default_config_string.as_bytes())?;

        log::info!("Created config file at {}", config_path.to_string_lossy());

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

async fn run_preset(
    state: Arc<Mutex<AppState>>,
    controller_id: u32,
    preset_id: usize,
    config: HashMap<String, toml::Value>,
) -> Result<(), RunError> {
    let controller = {
        let lock = state.lock().await;
        lock.client
            .get_controller(controller_id)
            .await
            .map_err(|e| RunError::OpenRGBError {
                message: format!("Failed to get controller ID {controller_id}"),
                error: e,
            })?
    };
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

    modes[preset_id].init(&config);
    loop {
        modes[preset_id].update(&mut screen);

        {
            let lock = state.lock().await;
            lock.client
                .update_leds(controller_id as u32, screen.clone())
                .await
                .map_err(|e| RunError::OpenRGBError {
                    message: format!("Failed to update LEDs for controller ID {controller_id}"),
                    error: e,
                })?;
        }

        tokio::time::sleep(Duration::from_nanos(10_000_000)).await;
    }
}

fn request_quit(state: Arc<Mutex<AppState>>) {
    state.blocking_lock().running = false;
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let settings = setup_config().unwrap_or_else(|e| {
        log::error!("Failed to load configuration: {}", e);
        exit(1);
    });

    log::info!("Loaded configuration successfully.");

    if settings.format_info.version != configuration::CURRENT_FORMAT_VERSION {
        log::warn!(
            "Configuration is outdated (v{} > v{}). Some things might break!",
            configuration::CURRENT_FORMAT_VERSION,
            settings.format_info.version
        )
    }

    if settings.controller_configs.len() == 0 {
        log::warn!(
            "Controller configuration is empty! Refer to README.md for configuration guides."
        )
    }

    log::info!("Available presets:");
    for (idx, preset) in all_presets().iter().enumerate() {
        log::info!("- ({}) {}", idx, preset.name())
    }

    let state = {
        let client = OpenRGB::connect().await.unwrap_or_else(|e| {
            log::error!("Failed to connect to OpenRGB server: {}", e);
            exit(1);
        });

        Arc::new(Mutex::new(AppState {
            client,
            running: true,
        }))
    };

    {
        let state = state.clone();
        ctrlc::set_handler(move || {
            request_quit(state.clone());
        })
        .expect("Error setting CTRL-C as handler");
    }

    let mut tasks = Vec::new();

    for (_controller_name, controller_config) in settings.controller_configs {
        let state = state.clone();

        tasks.push(tokio::spawn(async move {
            run_preset(
                state.clone(),
                controller_config.controller_id as u32,
                controller_config.selected_mode,
                controller_config.function_config,
            )
            .await
        }));
    }

    for (idx, task) in tasks.into_iter().enumerate() {
        let result = task.await.expect("Failed to join task");

        match result {
            Ok(()) => log::debug!("Task {idx} exited successfully."),
            Err(e) => {
                log::error!("Error occured during application runtime:");
                log::error!("  {e}")
            }
        }
    }
}
