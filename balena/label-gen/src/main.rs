//! Jenkins executor labels generator.
//!
//! Right now only information from USB events are used to generate labels.
//! In the future, other sources could be used.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use futures_util::future::ready;
use futures_util::stream::StreamExt;
use log::*;
use serde_derive::Deserialize;
use structopt::StructOpt;
use tokio_udev::{Context, EventType, MonitorBuilder};

/// Command line options.
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str), help = "Labels output file path")]
    output: PathBuf,
    #[structopt(default_value = "gen", short, help = "Label prefix")]
    prefix: String,
}

/// Configuration file.
#[derive(Debug, Deserialize)]
struct Config {
    devices: Vec<UsbConfig>,
}

#[derive(Debug, Deserialize)]
struct UsbConfig {
    id: String,
    label: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("{:?}", opt);

    // TODO: Add config path option
    let config_path = "./config.toml";
    let config = std::fs::read_to_string(config_path)
        .expect("Failed to read from config file.");
    let config: Config = toml::from_str(&config).unwrap();

    let mut labels = HashMap::new();
    labels.insert("/prefix".to_string(), "gen".to_string());

    // Add labels based on devices already connected
    let context = udev::Context::new().unwrap();
    let mut enumerator = udev::Enumerator::new(&context).unwrap();
    for device in enumerator.scan_devices().unwrap() {
        if let Some(label) = get_label(&config, &device) {
            let syspath = device.syspath().to_str().unwrap().to_string();

            if labels.values().find(|l| l == &&label).is_none() {
                info!("adding label {} for {}", label, syspath);
                labels.insert(syspath.clone(), label);
            } else {
                // TODO: Filter this out correctly
                warn!("attempt to add duplicate label {} for {}", label, syspath);
            }
        }
    }

    write_labels(&opt.output, &labels);

    let context = Context::new().unwrap();
    let mut builder = MonitorBuilder::new(&context).unwrap();
    builder
        .match_subsystem_devtype("usb", "usb_device")
        .unwrap();

    info!("Listening on udev events!");
    let monitor = builder.listen().unwrap();
    monitor
        .for_each(|event| {
            let device = event.device();
            trace!(
                "Hotplug event: {}: {}",
                event.event_type(),
                device.syspath().display()
            );
            let syspath = device.syspath().to_str().unwrap().to_string();

            match event.event_type() {
                EventType::Add => {
                    if let Some(label) = get_label(&config, &device) {
                        let syspath = device.syspath().to_str().unwrap().to_string();
                        info!("adding label {} for {}", label, syspath);
                        labels.insert(syspath.clone(), label);
                    }
                }
                EventType::Remove => {
                    let label = labels.remove(&syspath).unwrap();
                    info!("removing label '{}'", label);
                }
                _ => (),
            }
            write_labels(&opt.output, &labels);

            ready(())
        })
        .await
}

/// Get the label for a `udev::Device` if it is found in the configuration.
fn get_label(config: &Config, device: &udev::Device) -> Option<String> {
    let vendor = {
        let vendor = device
            .properties()
            .find(|a| a.name() == OsStr::new("ID_VENDOR_ID"));
        if vendor.is_none() {
            return None;
        }
        vendor.unwrap().value().to_str().unwrap().to_string()
    };
    let product = {
        let product = device
            .properties()
            .find(|a| a.name() == OsStr::new("ID_MODEL_ID"));
        if product.is_none() {
            return None;
        }
        product.unwrap().value().to_str().unwrap().to_string()
    };

    for usb_config in &config.devices {
        if usb_config.id == format!("{}:{}", vendor, product) {
            return Some(usb_config.label.clone());
        }
    }

    None
}

/// Blow away the old labels file and rewrite labels.
fn write_labels(output: &PathBuf, labels: &HashMap<String, String>) {
    let mut file = File::create(output).unwrap();
    labels
        .values()
        .for_each(|l| file.write_all(format!("{} ", l).as_bytes()).unwrap());
    file.flush().unwrap();
}
