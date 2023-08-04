use log::{error, info};
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
                let msg = String::from_utf8_lossy(&buffer[0..bytes_read]);
                info!("Received message: {}", msg);
            }
            Err(err) => {
                error!("Error reading message: {:?}", err);
                break;
            }
        }
    }
}
