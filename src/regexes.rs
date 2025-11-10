use regex::Regex;
use std::sync::OnceLock;

pub struct Regexes {
    pub row_end: Regex,
    pub isolate_chinese_section: Regex,
    pub pronunciation_sections: Regex,

    pub mc_section_start: Regex,
    pub mc_section_end: Regex,
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

    pub old_old_chinese_start: Regex,
    pub old_old_chinese: Regex,
    pub old_bs_section_start: Regex,
    pub old_bs_section_end: Regex,
    pub old_zh_section_start: Regex,
    pub old_zh_section_end: Regex,
    pub old_zh_filter: Regex,

    pub modern_data: Regex,
}

impl Regexes {
    pub fn new() -> Self {
        Self {
            row_end: Self::re(r#"</tr>"#),
            isolate_chinese_section: Self::re(r#"<h2 id=".*">(.*)</h2>"#),
            pronunciation_sections: Self::re(r#"<h[34] id="Pronunciation.*">(.*)</h[34]>"#),
            mc_section_start: Self::re(r#"title="w:Middle Chinese" class="extiw">Middle Chinese"#),
            mc_section_end: Self::re(r#"</tbody></table></div></div>"#),
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
            old_old_chinese_start: Self::re(r#"<small>Old<br/>Chinese</small>"#),
            old_old_chinese: Self::re(r#"<span class="IPAchar">/(.*)/.*</span>"#),
            old_bs_section_start: Self::re(
                r#"title="w:William H. Baxter" class="extiw">Baxter</a>"#,
            ),
            old_bs_section_end: Self::re(
                r#"</tbody></table>\n<table class="wikitable mw-collapsible mw-collapsed".*>"#,
            ),
            old_zh_section_start: Self::re(
                r#"title="w:Zhengzhang Shangfang" class="extiw">Zhengzhang</a> system \(2003\)"#,
            ),
            old_zh_section_end: Self::re(r#"</tbody></table></div></div></div></div><link"#),
            old_zh_filter: Self::re(r#"(\*.*)<.*>.*</.*>"#),
            modern_data: Self::re(r#"<div class="standard-box zhpron" .* data-mw='(.*)'"#),
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
