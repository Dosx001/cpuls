fn main() {
    let formatter = syslog::Formatter3164 {
        facility: syslog::Facility::LOG_USER,
        hostname: None,
        process: "cpuls".into(),
        pid: 0,
    };
    let logger = match syslog::unix(formatter) {
        Err(e) => {
            println!("impossible to connect to syslog: {:?}", e);
            return;
        }
        Ok(logger) => logger,
    };
    let _ = log::set_boxed_logger(Box::new(syslog::BasicLogger::new(logger)))
        .map(|()| log::set_max_level(log::LevelFilter::Trace));
    match cpuid::clock_frequency() {
        Some(frequency) => {
            if 3800 < frequency {
                log::error!("CPU frequency is too high: {}", frequency);
            } else if 3000 < frequency {
                log::warn!("CPU frequency is high: {}", frequency);
            } else {
                log::info!("CPU frequency: {}", frequency);
            }
        }
        None => log::error!("Failed to get CPU speed"),
    };
}
