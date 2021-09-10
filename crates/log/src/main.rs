use log::{info, warn, LevelFilter, Log};

fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();
    let mut yak = Yak("yak".to_string());
    shave_the_yak(&mut yak);
}

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{}: {} {}", record.level(), record.target(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

#[derive(Debug)]
struct Yak(String);

impl Yak {
    fn shave(&mut self, _: u32) {}
}

fn find_a_razor() -> Result<u32, u32> {
    Ok(1)
}

fn shave_the_yak(yak: &mut Yak) {
    info!(target: "yak_events", "Commencing yak shaving for {:?}", yak);
    loop {
        match find_a_razor() {
            Ok(razor) => {
                info!("Razor located: {}", razor);
                yak.shave(razor);
                break;
            }
            Err(err) => {
                warn!("Unable to locate a razor: {}, retrying", err);
            }
        }
    }
}
