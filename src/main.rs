use chrono::prelude::*;
use chrono_tz;

const DEFAULT_TZ: &str = "Europe/Berlin";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let timezone = args.last();

    let tz: chrono_tz::Tz = timezone
        .unwrap_or(&String::from(DEFAULT_TZ))
        .clone()
        .parse()
        .unwrap_or(DEFAULT_TZ.parse().unwrap());

    let now = Utc::now();
    let local = now.with_timezone(&tz);

    println!("UTC:   {}", now.to_rfc2822());
    println!("Local: {}", local.to_rfc2822());
}
