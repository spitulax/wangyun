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
    if new_section.len() > 0 {
        Some(new_section)
    } else {
        None
    }
}
