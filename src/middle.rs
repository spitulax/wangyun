use crate::{regexes::regexes, utils::regex_isolate_one};

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
    let re_start = &regexes().mc_section_start;
    let re_end = &regexes().mc_section_end;
    let mc_section = if let Some(s) = regex_isolate_one(section, re_start, re_end) {
        s
    } else {
        return vec![];
    };

    let mut datas = Vec::<Data<'_>>::new();
    let readings = fetch_readings(mc_section);
    for reading in readings {
        let data = Data {
            reading,
            ..Data::default()
        };
        datas.push(data);
    }

    datas
}

pub fn fetch_readings(section: &str) -> Vec<&str> {
    let re_row_start = &regexes().mc_reading_start;
    let re_row_end = &regexes().mc_reading_end;
    let re_reading = &regexes().mc_reading;
    let mut readings = Vec::<&str>::new();
    if let Some(row) = regex_isolate_one(section, re_row_start, re_row_end) {
        for (_, [reading]) in re_reading.captures_iter(row).map(|c| c.extract()) {
            readings.push(reading);
        }

        if readings.is_empty() {
            unreachable!("Misformatted HTML: Reading row should be filled.")
        }
    } else {
        unreachable!("Misformatted HTML: Middle Chinese section should have the reading row");
    }

    readings
}
