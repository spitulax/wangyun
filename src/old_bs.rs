use crate::{regexes, utils::fetch_row, utils::regex_isolate_one};

#[derive(Debug, Default)]
pub struct Data<'a> {
    pub reading: &'a str,
    pub old_chinese: &'a str,
}

pub fn fetch(section: &'_ str) -> Vec<Data<'_>> {
    let re_start = &regexes().old_bs_section_start;
    let re_end = &regexes().old_bs_section_end;
    let old_section = if let Some(s) = regex_isolate_one(section, re_start, re_end) {
        s
    } else {
        return vec![];
    };

    let mut datas = vec![];

    let readings = fetch_row(
        old_section,
        &regexes().mc_reading_start,
        &regexes().mc_simple_row,
    );
    for reading in readings {
        datas.push(Data {
            reading,
            ..Data::default()
        });
    }

    let old_chineses = fetch_row(
        old_section,
        &regexes().old_old_chinese_start,
        &regexes().old_old_chinese,
    );
    for (data, old_chinese) in datas.iter_mut().zip(old_chineses.into_iter()) {
        data.old_chinese = old_chinese;
    }

    datas
}
