use regex::Regex;

use crate::utils::regex_isolate_one;

#[derive(Debug, Default)]
pub struct Data<'a> {
    reading: &'a str,
    init: &'a str,
    fin: &'a str,
    tone: Tones,
    open: bool,
    division: usize,
    fanqie: &'a str,
    baxter: &'a str,
    mandarin: &'a str,
    cantonese: String,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Tones {
    #[default]
    Unknown,
    Ping,
    Shang,
    Qu,
    Ru,
}

pub fn fetch(section: &'_ str) -> Vec<Data<'_>> {
    // FIXME: Compiling regex in loop
    let re_start = Regex::new(r#"title="w:Middle Chinese" class="extiw">Middle Chinese"#)
        .expect("invalid regex");
    let re_end = Regex::new(r#"<div class="vsSwitcher" data-toggle-category="pronunciations">"#)
        .expect("invalid regex");
    let mc_section = if let Some(s) = regex_isolate_one(section, &re_start, &re_end) {
        s
    } else {
        return vec![];
    };

    let mut datas = Vec::<Data<'_>>::new();
    let readings = fetch_readings(mc_section);
    for reading in readings {
        let mut data = Data::default();
        data.reading = reading;
        datas.push(data);
    }

    datas
}

pub fn fetch_readings(section: &str) -> Vec<&str> {
    // FIXME: Compiling regex in loop
    let re_row_start =
        Regex::new(r#"<th.*><small>Reading #</small></th>\n"#).expect("invalid regex");
    let re_row_end = Regex::new(r#"</tr>"#).expect("invalid regex");
    let re_reading = Regex::new(r#"<td.*>(.*)</td>"#).expect("invalid regex");
    let mut readings = Vec::<&str>::new();
    if let Some(row) = regex_isolate_one(section, &re_row_start, &re_row_end) {
        for (_, [reading]) in re_reading.captures_iter(row).map(|c| c.extract()) {
            readings.push(reading);
        }

        if readings.len() <= 0 {
            unreachable!("Misformatted HTML: Reading row should be filled.")
        }
    } else {
        unreachable!("Misformatted HTML: Middle Chinese section should have the reading row");
    }

    readings
}
