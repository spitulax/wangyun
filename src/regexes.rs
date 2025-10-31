use regex::Regex;
use std::sync::OnceLock;

pub struct Regexes {
    pub isolate_chinese_section: Regex,
    pub pronunciation_sections: Regex,

    pub mc_section_start: Regex,
    pub mc_section_end: Regex,
    pub mc_row_end: Regex,
    pub mc_simple_row: Regex,
    pub mc_reading_start: Regex,
    pub mc_init_start: Regex,
    pub mc_fin_start: Regex,
    pub mc_init_fin: Regex,
    pub mc_tone_start: Regex,
    pub mc_tone: Regex,
    pub mc_open_start: Regex,
    pub mc_div_start: Regex,
    pub mc_fanqie_start: Regex,
    pub mc_fanqie: Regex,
    pub mc_baxter_start: Regex,
    pub mc_baxter: Regex,
    pub mc_mandarin_start: Regex,
    pub mc_cantonese_start: Regex,
    pub mc_mandarin: Regex,
    pub mc_cantonese: Regex,
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
            mc_row_end: Self::re(r#"</tr>"#),
            mc_simple_row: Self::re(r#"<td.*>(.*)</td>"#),
            mc_reading_start: Self::re(r#"<th.*><small>Reading #</small></th>\n"#),
            mc_init_start: Self::re(r#"<th.*><small>Initial</small>"#),
            mc_fin_start: Self::re(r#"<th.*><small>Final</small>"#),
            mc_init_fin: Self::re(r#"title="Appendix:Middle Chinese">(.*)</a></span>"#),
            mc_tone_start: Self::re(r#"<th.*><small>Tone</small>"#),
            mc_tone: Self::re(r#"<td.*>(.*) .*</td>"#),
            mc_open_start: Self::re(r#"<th.*><small>Openness</small>"#),
            mc_div_start: Self::re(r#"<th.*><small>Division</small>"#),
            mc_fanqie_start: Self::re(r#"title="w:Fanqie" class="extiw">Fanqie"#),
            mc_fanqie: Self::re(r#"title=".*">(.*)</a><.*title=".*">(.*)</a>"#),
            mc_baxter_start: Self::re(
                r#"title="w:Baxter's transcription for Middle Chinese" class="extiw">Baxter"#,
            ),
            mc_baxter: Self::re(r#"<span lang="zh">(.*)</span>"#),
            mc_mandarin_start: Self::re(r#"<small>Expected<br/>Mandarin<br/>Reflex</small>"#),
            mc_cantonese_start: Self::re(r#"<small>Expected<br/>Cantonese<br/>Reflex</small>"#),
            mc_mandarin: Self::re(r#"<td.*>(.*)</td>"#),
            mc_cantonese: Self::re(r#"<td.*>(.*<sup>.</sup>)</td>"#),
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
