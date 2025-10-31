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
