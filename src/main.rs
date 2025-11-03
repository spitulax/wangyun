use clap::{Parser, ValueEnum};

use regexes::regexes;

mod middle;
mod modern;
mod old_bs;
mod old_zh;
mod regexes;
mod utils;

const _USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/spitulax/wangyun; bintangadiputra@proton.me)",
);

type Result<T> = std::result::Result<T, ()>;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Characters to look up
    chars: String,

    /// Show Middle Chinese rime info
    #[arg(short, long)]
    middle: bool,

    /// Show Old Chinese rime info
    #[arg(short, long)]
    old: bool,

    /// Show pronunciations of modern variants
    #[arg(short('M'), long)]
    modern: Vec<ModernVariants>,
}

#[derive(Debug, Copy, Clone, ValueEnum, PartialEq, Eq)]
enum ModernVariants {
    All,
    Mandarin,
    Cantonese,
    Gan,
    Hakka,
    Jin,
    Min,
    Pinghua,
    Wu,
    Xiang,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let page = request();
    let section = isolate_chinese_section(&page);

    for _char in args.chars.chars() {
        let pronunciations = pronunciation_sections(section);

        for pronunciation in pronunciations {
            if args.middle {
                let data = middle::fetch(pronunciation);
                println!("Middle Chinese: {data:#?}");
            }

            if args.old {
                let data_bs = old_bs::fetch(pronunciation);
                println!("Old Chinese (B-S): {data_bs:#?}");

                let data_zh = old_zh::fetch(pronunciation);
                println!("Old Chinese (Zhengzhang): {data_zh:#?}");
            }

            if !args.modern.is_empty() {
                let data = modern::fetch(pronunciation);
                println!("Modern: {data:#?}");
            }

            println!("\n");
        }
    }

    Ok(())
}

fn request() -> String {
    include_str!("../page.html").to_string()
}

fn isolate_chinese_section(page: &str) -> &str {
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

fn pronunciation_sections(section: &str) -> Vec<&str> {
    let re = &regexes().pronunciation_sections;
    let mut locs = re.capture_locations();
    let mut pron_sections = Vec::<&str>::new();
    let mut offset = 0;
    let mut section_locs = (0, 0);
    let mut search_end = false;
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
