use crate::{
    regexes::regexes,
    utils::{fetch_row, regex_isolate_one},
};

#[derive(Debug, Default)]
pub struct Data<'a> {
    reading: &'a str,
    init: &'a str,
    fin: &'a str,
    tone: Tones,
    open: bool,
    division: usize,
    fanqie: String,
    baxter: &'a str,
    expected_mandarin: &'a str,
    expected_cantonese: String,
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

    let mut datas = vec![];

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

    let fanqies = fetch_fanqie(mc_section);
    for (data, fanqie) in datas.iter_mut().zip(fanqies.into_iter()) {
        data.fanqie = fanqie;
    }

    let baxters = fetch_row(mc_section, &regexes().mc_baxter_start, &regexes().mc_baxter);
    for (data, baxter) in datas.iter_mut().zip(baxters.iter()) {
        data.baxter = baxter;
    }

    let expected_mandarins = fetch_row(
        mc_section,
        &regexes().mc_mandarin_start,
        &regexes().mc_mandarin,
    );
    for (data, expected_mandarin) in datas.iter_mut().zip(expected_mandarins.iter()) {
        data.expected_mandarin = expected_mandarin;
    }

    let expected_cantoneses = fetch_row(
        mc_section,
        &regexes().mc_cantonese_start,
        &regexes().mc_cantonese,
    );
    for (data, expected_cantonese) in datas.iter_mut().zip(expected_cantoneses.iter()) {
        let expected = expected_cantonese
            .replace("<sup>", "")
            .replace("</sup>", "");
        data.expected_cantonese = expected;
    }

    datas
}

pub fn fetch_fanqie(section: &str) -> Vec<String> {
    let re_row_start = &regexes().mc_fanqie_start;
    let re_row_end = &regexes().row_end;
    let re_row_elem = &regexes().mc_fanqie;
    let mut elems = Vec::<String>::new();
    if let Some(row) = regex_isolate_one(section, re_row_start, re_row_end) {
        for (_, [f1, f2]) in re_row_elem.captures_iter(row).map(|c| c.extract()) {
            let mut elem = String::new();
            elem.push_str(f1);
            elem.push_str(f2);
            elem.push('切');
            elems.push(elem);
        }

        if elems.is_empty() {
            unreachable!("Misformatted HTML: Row should be filled.");
        }
    } else {
        unreachable!("Misformatted HTML: Middle Chinese section should have the specified row");
    }

    elems
}
