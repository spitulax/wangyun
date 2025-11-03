use reqwest::header::USER_AGENT;
use std::{
    sync::OnceLock,
    thread::sleep,
    time::{Duration, SystemTime},
};

const REQUEST_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/spitulax/wangyun; bintangadiputra@proton.me)",
);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
// TODO: Should we async?
// NOTE: Since we use blocking, 100ms gap per request should be fine to cap the requests to 10 per
// second as for Wikimedia's requirement. This results in slower requests however, since when we
// request 10 times or fewer it should also met the requirement even without this delay.
const REQUEST_PER_SECOND: u64 = 10;
const REQUEST_GAP: Duration = Duration::from_millis(1000 / REQUEST_PER_SECOND);
static LAST_REQUEST_TIME: OnceLock<SystemTime> = OnceLock::new();

pub fn request(c: char) -> reqwest::Result<String> {
    if let Some(t) = LAST_REQUEST_TIME.get() {
        let dur = SystemTime::now()
            .duration_since(*t)
            .expect("incorrect time");
        if dur < REQUEST_GAP {
            sleep(REQUEST_GAP - dur);
        }
    }

    let client = reqwest::blocking::Client::new();
    eprintln!("Requesting {}...", c);
    let response = client
        .get(format!(
            "https://en.wiktionary.org/api/rest_v1/page/html/{}",
            c
        ))
        .header(USER_AGENT, REQUEST_USER_AGENT)
        .timeout(REQUEST_TIMEOUT)
        .send()?
        .text();

    LAST_REQUEST_TIME.get_or_init(|| SystemTime::now());

    response
}
