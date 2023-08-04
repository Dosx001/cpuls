use log::{debug, error, info, warn};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

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
    for stream in TcpListener::bind("127.0.0.1:4444").unwrap().incoming() {
        match stream {
            Ok(stream) => {
                spawn(|| handle_client(stream));
            }
            Err(err) => {
                error!("Error accepting incoming connection: {:?}", err);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(bytes_read) => {
                let msg = String::from_utf8_lossy(&buffer[0..bytes_read]).to_string();
                match msg.trim() {
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
                    _ => error!("Unknown message: {}", msg),
                }
            }
            Err(err) => {
                error!("Error reading message: {:?}", err);
                break;
            }
        }
    }
}
