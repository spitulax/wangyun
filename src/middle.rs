use regex::Regex;

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

impl From<&str> for Tones {
    fn from(s: &str) -> Self {
        match s {
            "Level" => Self::Ping,
            "Rising" => Self::Shang,
            "Departing" => Self::Qu,
            "Checked" => Self::Ru,
            _ => unreachable!("Invalid tone name."),
        }
    }
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
    let readings = fetch_row(
        mc_section,
        &regexes().mc_reading_start,
        &regexes().mc_simple_row,
    );
    for reading in readings {
        datas.push(Data {
            reading,
            ..Data::default()
        });
    }

    let inits = fetch_row(mc_section, &regexes().mc_init_start, &regexes().mc_init_fin);
    for (data, init) in datas.iter_mut().zip(inits.iter()) {
        data.init = init;
    }

    let fins = fetch_row(mc_section, &regexes().mc_fin_start, &regexes().mc_init_fin);
    for (data, fin) in datas.iter_mut().zip(fins.iter()) {
        data.fin = fin;
    }

    let tones = fetch_row(mc_section, &regexes().mc_tone_start, &regexes().mc_tone);
    for (data, tone) in datas.iter_mut().zip(tones.iter()) {
        data.tone = (*tone).into();
    }

    let opens = fetch_row(
        mc_section,
        &regexes().mc_open_start,
        &regexes().mc_simple_row,
    );
    for (data, open) in datas.iter_mut().zip(opens.iter()) {
        data.open = match *open {
            "Open" => true,
            "Closed" => false,
            _ => unreachable!("Invalid openness `{}`", *open),
        }
    }

    let divs = fetch_row(
        mc_section,
        &regexes().mc_div_start,
        &regexes().mc_simple_row,
    );
    for (data, div) in datas.iter_mut().zip(divs.iter()) {
        data.division = match *div {
            "I" => 1,
            "II" => 2,
            "III" => 3,
            "IV" => 4,
            _ => unreachable!("Invalid division `{}`", *div),
        }
    }

    datas
}

pub fn fetch_row<'a>(section: &'a str, re_row_start: &Regex, re_row_elem: &Regex) -> Vec<&'a str> {
    let re_row_end = &regexes().mc_row_end;
    let mut elems = Vec::<&str>::new();
    if let Some(row) = regex_isolate_one(section, re_row_start, re_row_end) {
        for (_, [reading]) in re_row_elem.captures_iter(row).map(|c| c.extract()) {
            elems.push(reading);
        }

        if elems.is_empty() {
            unreachable!("Misformatted HTML: Row should be filled.")
        }
    } else {
        unreachable!("Misformatted HTML: Middle Chinese section should have the specified row");
    }

    elems
}
