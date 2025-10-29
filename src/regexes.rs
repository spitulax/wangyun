use regex::Regex;
use std::sync::OnceLock;

pub struct Regexes {
    pub isolate_chinese_section: Regex,
    pub pronunciation_sections: Regex,

    pub mc_section_start: Regex,
    pub mc_section_end: Regex,
    pub mc_reading_start: Regex,
    pub mc_reading_end: Regex,
    pub mc_reading: Regex,
}

impl Regexes {
    pub fn new() -> Self {
        Self {
            isolate_chinese_section: Self::re(r#"<h2 id=".*">(.*)</h2>"#),
            pronunciation_sections: Self::re(r#"<h3 id=".*">(.*)</h3>"#),
            mc_section_start: Self::re(r#"title="w:Middle Chinese" class="extiw">Middle Chinese"#),
            mc_section_end: Self::re(
                r#"<div class="vsSwitcher" data-toggle-category="pronunciations">"#,
            ),
            mc_reading_start: Self::re(r#"<th.*><small>Reading #</small></th>\n"#),
            mc_reading_end: Self::re(r#"</tr>"#),
            mc_reading: Self::re(r#"<td.*>(.*)</td>"#),
        }
    }

    fn re(re: &str) -> Regex {
        Regex::new(re).expect("invalid regex")
    }
}

pub static REGEXES: OnceLock<Regexes> = OnceLock::new();
pub fn regexes() -> &'static Regexes {
    REGEXES.get_or_init(Regexes::new)
}
