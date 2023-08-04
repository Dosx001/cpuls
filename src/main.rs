use log::{debug, error, info, warn};
use std::io::{self, Read};

fn main() {
    let _ = log::set_boxed_logger(Box::new(syslog::BasicLogger::new(
        match syslog::unix(syslog::Formatter3164 {
            facility: syslog::Facility::LOG_USER,
            hostname: None,
            process: "cpuls".into(),
            pid: 0,
        }) {
            Err(e) => {
                println!("impossible to connect to syslog: {:?}", e);
                return;
            }
            Ok(logger) => logger,
        },
    )))
    .map(|()| log::set_max_level(log::LevelFilter::Trace));
    let mut buffer = [0; 512];
    loop {
        let bytes_read = io::stdin().read(&mut buffer);
        match bytes_read {
            Ok(bytes_read) => match std::str::from_utf8(&buffer[0..bytes_read]) {
                Ok(request_str) => match request_str.trim() {
                    "info" => match cpuid::identify() {
                        Ok(info) => {
                            info!("Brand: {}", info.brand);
                            info!("Vendor: {}", info.vendor);
                            info!("Codename: {}", info.codename);
                        }
                        Err(e) => error!("Error getting brand: {:?}", e),
                    },
                    "clock" => match cpuid::clock_frequency() {
                        Some(frequency) => {
                            if 3800 < frequency {
                                error!("CPU frequency is too high: {}", frequency);
                            } else if 3000 < frequency {
                                warn!("CPU frequency is high: {}", frequency);
                            } else {
                                info!("CPU frequency: {}", frequency);
                            }
                        }
                        None => error!("Failed to get CPU speed"),
                    },
                    "error" => debug!("{}", cpuid::error()),
                    "present" => {
                        if cpuid::is_present() {
                            info!("CPU is present");
                        } else {
                            error!("CPU is not present");
                        }
                    }
                    "version" => info!("Version: {}", cpuid::version()),
                    _ => error!("Unknown message: {}", request_str),
                },
                Err(e) => error!("Error reading from stdin: {:?}", e),
            },
            Err(e) => {
                error!("Error reading from stdin: {:?}", e);
            }
        }
    }
}
