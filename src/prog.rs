use std::collections::HashSet;

use crate::{
    middle, modern, old_bs, old_zh,
    request::request,
    utils::{isolate_chinese_section, pronunciation_sections},
    Args, Variants,
};

macro_rules! print_modern {
    ($args: expr, $enum: expr, $prereq: expr, $val: expr, $text: literal) => {
        if $args.modern.contains(&$enum) || ($prereq) {
            if let Some(ref s) = $val {
                println!(concat!("\t\t", $text, ": {}"), s);
            }
        }
    };
}

pub fn display(args: &Args) -> reqwest::Result<()> {
    let mut pages = vec![];
    for c in args.chars.chars() {
        pages.push(request(c)?);
    }

    for (i, c) in args.chars.chars().enumerate() {
        let page = &pages[i];
        let section = isolate_chinese_section(page);

        if i > 0 {
            println!();
        }

        println!("\x1b[31;1mCharacter: {c}\x1b[0m");

        let pronunciations = pronunciation_sections(section);

        for (i, pronunciation) in pronunciations.iter().enumerate() {
            println!("\x1b[32;1mPronunciation {}:\x1b[0m", i + 1);

            if args.middle {
                let data = middle::fetch(pronunciation);
                for r in data {
                    let openness = if r.open { "開" } else { "合" };
                    let division = match r.division {
                        1 => "一",
                        2 => "二",
                        3 => "三",
                        4 => "四",
                        _ => "〇",
                    };

                    println!("\t\x1b[33;1mMiddle Chinese (Reading {}):\x1b[0m", r.reading);
                    println!(
                        "\t\tRime: {}{}{}{}{}",
                        r.init, r.fin, r.tone, openness, division
                    );
                    println!("\t\tFanqie: {}", r.fanqie);
                    println!("\t\tBaxter: \x1b[34;1m{}\x1b[0m", r.baxter);
                    println!("\t\tExpected Mandarin Reflex: {}", r.expected_mandarin);
                    println!("\t\tExpected Cantonese Reflex: {}", r.expected_cantonese);
                }
            }

            if args.old {
                let data_bs = old_bs::fetch(pronunciation);
                if !data_bs.is_empty() {
                    println!("\t\x1b[33;1mOld Chinese (Baxter-Sagart):\x1b[0m");
                    for r in data_bs {
                        println!(
                            "\t\tReading {}: \x1b[34;1m{}\x1b[0m",
                            r.reading, r.old_chinese
                        );
                    }
                }

                let data_zh = old_zh::fetch(pronunciation);
                if !data_zh.is_empty() {
                    println!("\t\x1b[33;1mOld Chinese (Zhengzhang):\x1b[0m");
                    for r in data_zh {
                        println!(
                            "\t\tReading {}: \x1b[34;1m{}\x1b[0m",
                            r.reading, r.old_chinese
                        );
                    }
                }
            }

            if !args.modern.is_empty() {
                let data = modern::fetch(pronunciation);
                let all = args.modern.contains(&Variants::All);
                let man = all || args.modern.contains(&Variants::Man);
                let can = all || args.modern.contains(&Variants::Can);
                let hak = all || args.modern.contains(&Variants::Hak);
                let min = all || args.modern.contains(&Variants::Min);
                let minn = all || min || args.modern.contains(&Variants::Mins);
                let wu = all || args.modern.contains(&Variants::Wu);
                let xiang = all || args.modern.contains(&Variants::Xiang);

                println!("\t\x1b[33;1mModern Pronunciations:\x1b[0m");

                print_modern!(
                    args,
                    Variants::Ms,
                    man,
                    data.ma_standard,
                    "Mandarin (Standard, Pinyin)"
                );
                print_modern!(
                    args,
                    Variants::Mc,
                    man,
                    data.ma_chengdu,
                    "Mandarin (Chengdu, Sichuanese Pinyin)"
                );
                print_modern!(
                    args,
                    Variants::Mx,
                    man,
                    data.ma_xian,
                    "Mandarin (Xi'an, Guanzhong Pinyin)"
                );
                print_modern!(
                    args,
                    Variants::Mn,
                    man,
                    data.ma_nanjing,
                    "Mandarin (Nanjing, Nanjing Pinyin)"
                );
                print_modern!(
                    args,
                    Variants::Md,
                    man,
                    data.ma_dungan,
                    "Mandarin (Dungan, Cyrillic)"
                );

                print_modern!(
                    args,
                    Variants::Cg,
                    can,
                    data.ca_guangzhou,
                    "Cantonese (Guangzhou-Hong Kong, Jyutping)"
                );
                print_modern!(
                    args,
                    Variants::Cd,
                    can,
                    data.ca_dongguan,
                    "Cantonese (Dongguan, Jyutping++)"
                );
                print_modern!(
                    args,
                    Variants::Ct,
                    can,
                    data.ca_taishan,
                    "Cantonese (Taishan, Wiktionary)"
                );
                print_modern!(
                    args,
                    Variants::Cy,
                    can,
                    data.ca_yangjiang,
                    "Cantonese (Yangjiang, Jyutping++)"
                );

                print_modern!(args, Variants::Gan, all, data.gan, "Gan (Wiktionary)");

                print_modern!(
                    args,
                    Variants::Hs,
                    hak,
                    data.ha_sixian,
                    "Hakka (Sixian, Pha̍k-fa-sṳ)"
                );
                print_modern!(
                    args,
                    Variants::Hh,
                    hak,
                    data.ha_hailu,
                    "Hakka (Hailu, Taiwanese Hakka Romanization)"
                );
                print_modern!(
                    args,
                    Variants::Hm,
                    hak,
                    data.ha_meixian,
                    "Hakka (Meixian, Guangdong Romanization)"
                );
                print_modern!(
                    args,
                    Variants::Hc,
                    hak,
                    data.ha_changting,
                    "Hakka (Changting, Changting Pinyin)"
                );

                print_modern!(args, Variants::Jin, all, data.jin, "Jin (Wiktionary)");

                print_modern!(
                    args,
                    Variants::Minn,
                    min,
                    data.mi_northern,
                    "Northern Min (Gṳ̿ing-nǎing Lô̤-mǎ-cī)"
                );
                print_modern!(
                    args,
                    Variants::Mine,
                    min,
                    data.mi_eastern,
                    "Eastern Min (Bàng-uâ-cê)"
                );
                print_modern!(
                    args,
                    Variants::Minp,
                    min,
                    data.mi_puxian,
                    "Puxian Min (Pouseng Ping'ing)"
                );
                print_modern!(
                    args,
                    Variants::Minh,
                    minn,
                    data.mi_hokkien,
                    "Hokkien (Pe̍h-ōe-jī)"
                );
                print_modern!(
                    args,
                    Variants::Mint,
                    minn,
                    data.mi_teochew,
                    "Teochew (Peng'im)"
                );
                print_modern!(
                    args,
                    Variants::Minl,
                    minn,
                    data.mi_leizhou,
                    "Leizhou (Leizhou Pinyin)"
                );

                print_modern!(
                    args,
                    Variants::Sp,
                    all,
                    data.ph_southern,
                    "Southern Pinghua (Jyutping++)"
                );

                print_modern!(
                    args,
                    Variants::Wn,
                    wu,
                    data.wu_northern,
                    "Wu (Northern, Wugniu)"
                );
                print_modern!(
                    args,
                    Variants::Wj,
                    wu,
                    data.wu_jinhua,
                    "Wu (Jinhua, Wugniu)"
                );

                print_modern!(
                    args,
                    Variants::Xc,
                    xiang,
                    data.xi_changsa,
                    "Xiang (Changsha, Wiktionary)"
                );
                print_modern!(
                    args,
                    Variants::Xl,
                    xiang,
                    data.xi_loudi,
                    "Xiang (Loudi, Wiktionary)"
                );
                print_modern!(
                    args,
                    Variants::Xh,
                    xiang,
                    data.xi_hengyang,
                    "Xiang (Hengyang, Wiktionary)"
                );
            }
        }
    }

    Ok(())
}

pub fn baxter(args: &Args) -> reqwest::Result<()> {
    let mut pages = vec![];
    for c in args.chars.chars() {
        pages.push(request(c)?);
    }

    for (i, _) in args.chars.chars().enumerate() {
        let page = &pages[i];
        let section = isolate_chinese_section(page);

        if i > 0 {
            print!(" ");
        }

        let mut list = vec![];

        let pronunciations = pronunciation_sections(section);
        for pronunciation in pronunciations {
            let data = middle::fetch(pronunciation);
            for r in data {
                list.push(r.baxter);
            }
        }

        let mut seen = HashSet::new();
        let list_uniq = list
            .into_iter()
            .filter(|item| seen.insert(*item))
            .collect::<Vec<_>>();

        if list_uniq.is_empty() {
            print!("[]");
        } else {
            print!("{}", list_uniq.join("|"));
        }
    }

    println!();

    Ok(())
}
