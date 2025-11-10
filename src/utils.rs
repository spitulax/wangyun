use crate::regexes;
use regex::Regex;

pub fn regex_isolate_one<'a>(
    section: &'a str,
    re_start: &Regex,
    re_end: &Regex,
) -> Option<&'a str> {
    let mut locs = re_start.capture_locations();
    let mut section_locs = (0, 0);
    if re_start.captures_read(&mut locs, section).is_some() {
        if let Some(loc) = locs.get(0) {
            section_locs.0 = loc.1;
            if re_end.captures_read_at(&mut locs, section, loc.1).is_some() {
                if let Some(loc) = locs.get(0) {
                    section_locs.1 = loc.0;
                }
            } else {
                unreachable!("Misformatted HTML: Section does not end.");
            }
        }
    }

    let new_section = section
        .get(section_locs.0..section_locs.1)
        .expect("invalid substring");
    if !new_section.is_empty() {
        Some(new_section)
    } else {
        None
    }
}

pub fn fetch_row<'a>(section: &'a str, re_row_start: &Regex, re_row_elem: &Regex) -> Vec<&'a str> {
    let re_row_end = &regexes().row_end;
    let mut elems = Vec::<&str>::new();
    if let Some(row) = regex_isolate_one(section, re_row_start, re_row_end) {
        for (_, [reading]) in re_row_elem.captures_iter(row).map(|c| c.extract()) {
            elems.push(reading);
        }

        if elems.is_empty() {
            unreachable!("Misformatted HTML: Row should be filled.");
        }
    } else {
        unreachable!("Misformatted HTML: Section should have the specified row");
    }

    elems
}

pub fn isolate_chinese_section(page: &str) -> &str {
    let re = &regexes().isolate_chinese_section;
    let mut locs = re.capture_locations();
    let mut chinese_section_locs = (0, 0);
    let mut offset = 0;
    let mut found_chinese_section = false;
    while re.captures_read_at(&mut locs, page, offset).is_some() {
        if let (Some(loc), Some(name_loc)) = (locs.get(0), locs.get(1)) {
            if found_chinese_section {
                chinese_section_locs.1 = loc.0;
                break;
            }
            if let Some("Chinese") = page.get(name_loc.0..name_loc.1) {
                found_chinese_section = true;
                chinese_section_locs.0 = loc.1;
            }
            offset = loc.1;
        }
    }
    if chinese_section_locs.1 < chinese_section_locs.0 {
        chinese_section_locs.1 = page.len();
    }

    if !found_chinese_section {
        panic!("no Chinese section found");
    }

    page.get(chinese_section_locs.0..chinese_section_locs.1)
        .expect("invalid substring")
}

pub fn pronunciation_sections(section: &str) -> Vec<&str> {
    let re = &regexes().pronunciation_sections;
    let mut pron_sections = Vec::<&str>::new();
    let mut offset = 0;
    let mut section_locs = (0, 0);
    let mut search_end = false;
    let mut locs = re.capture_locations();

    while re.captures_read_at(&mut locs, section, offset).is_some() {
        if let (Some(loc), Some(name_loc)) = (locs.get(0), locs.get(1)) {
            let name = section
                .get(name_loc.0..name_loc.1)
                .expect("invalid substring");
            if search_end {
                section_locs.1 = loc.0;
                pron_sections.push(
                    section
                        .get(section_locs.0..section_locs.1)
                        .expect("invalid substring"),
                );
                search_end = false;
            }
            if name.starts_with("Pronunciation") {
                search_end = true;
                section_locs.0 = loc.1;
            }
            offset = loc.1;
        }
    }
    if section_locs.1 < section_locs.0 {
        section_locs.1 = section.len();
        pron_sections.push(
            section
                .get(section_locs.0..section_locs.1)
                .expect("invalid substring"),
        );
    }

    pron_sections
}
