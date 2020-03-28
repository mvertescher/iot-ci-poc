//! Jenkins executor labels generator.
//!
//! Right now only information from USB events are used to generate labels.
//! In the future, other sources could be used.

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use futures_util::future::ready;
use futures_util::stream::StreamExt;
use log::*;
use tokio_udev::{Context, EventType, MonitorBuilder};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str), help = "Labels output file path")]
    output: PathBuf,
    #[structopt(default_value = "gen", short, help = "Label prefix")]
    prefix: String
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("{:?}", opt);

    let mut labels = HashMap::new();
    labels.insert("/prefix".to_string(), "gen".to_string());

    // TODO: On startup, inspect USB devices and add to labels.
    write_labels(&opt.output, &labels);

    let context = Context::new().unwrap();
    let mut builder = MonitorBuilder::new(&context).unwrap();
    builder.match_subsystem_devtype("usb", "usb_device").unwrap();

    info!("Listening on udev events!");
    let monitor = builder.listen().unwrap();
    monitor.for_each(|event| {
        let device = event.device();
        info!("Hotplug event: {}: {}", event.event_type(),
            device.syspath().display());
        let syspath = device.syspath().to_str().unwrap().to_string();

        match event.event_type() {
            EventType::Add => {
                for attribute in device.attributes() {
                    let attr = attribute.name().to_str().unwrap();
                    if attr == "product" {
                        let value = to_label(attribute.value().unwrap());

                        info!("adding label {}", value);
                        labels.insert(syspath.clone(), value);
                    }
                }
            }
            EventType::Remove => {
                // info!("removing label {}", value);
                labels.remove(&syspath);
            }
            _ => (),
        }
        write_labels(&opt.output, &labels);

        ready(())
    }).await
}

/// Blow away the old labels file and rewrite labels.
fn write_labels(output: &PathBuf, labels: &HashMap<String, String>) {
    let mut file = File::create(output).unwrap();
    labels.values().for_each(|l|
        file.write_all(format!("{} ", l).as_bytes()).unwrap());
    file.flush().unwrap();
}

/// Convert OS strings to labels.
fn to_label(value: &std::ffi::OsStr) -> String {
    let value = value.to_str().unwrap();
    let mut value = value.to_string();
    value.make_ascii_lowercase();
    let value = value.replace(" ", "");

    value
}
