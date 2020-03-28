//! Jenkins executor labels generator.
//!
//! Right now only information from USB events are used to generate labels.
//! In the future, other sources could be used.

use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

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

    let mut labels = vec![format!("{}", opt.prefix)];

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

        for attribute in device.attributes() {
            let attr = attribute.name().to_str().unwrap();
            if attr == "product" {
                let value = attribute.value().unwrap().to_str().unwrap();
                trace!("{} = {}", attr, value);

                // Format value
                let mut value = value.to_string();
                value.make_ascii_lowercase();
                let value = value.replace(" ", "");

                match event.event_type() {
                    EventType::Add => {
                        info!("adding label {}", value);
                        labels.push(value);
                    }
                    EventType::Remove => {
                        // FIXME: We should keep track of and trigger on syspaths.
                        info!("removing label {}", value);
                        labels = labels.iter()
                            .filter(|l| l.contains(&value))
                            .map(|l| l.to_string())
                            .collect();
                    }
                    _ => (),
                }

                write_labels(&opt.output, &labels);
            }
        }

        ready(())
    }).await
}

/// Blow away the old labels file and rewrite labels.
fn write_labels(output: &PathBuf, labels: &Vec<String>) {
    let mut file = File::create(output).unwrap();
    labels.iter().for_each(|l|
        file.write_all(format!("{} ", l).as_bytes()).unwrap());
    file.flush().unwrap();
}
