use crate::regexes::regexes;
use clap::ValueEnum;
use serde_json::{self, Value};

#[derive(Debug, Copy, Clone, ValueEnum, PartialEq, Eq)]
pub enum Variants {
    All,
    /// Mandarin (All)
    Man,
    /// Mandarin (Standard, Pinyin)
    Ms,
    /// Mandarin (Chengdu, Sichuanese Pinyin)
    Mc,
    /// Mandarin (Xi'an, Guanzhong Pinyin)
    Mx,
    /// Mandarin (Nanjing, Nanjing Pinyin)
    Mn,
    /// Mandarin (Dungan, Cyrillic)
    Md,
    /// Cantonese (All)
    Can,
    /// Cantonese (Guangzhou-Hong Kong, Jyutping)
    Cg,
    /// Cantonese (Dongguan, Jyutping++)
    Cd,
    /// Cantonese (Taishan, Wiktionary)
    Ct,
    /// Cantonese (Yangjiang, Jyutping++)
    Cy,
    /// Gan (Wiktionary)
    Gan,
    /// Hakka (All)
    Hak,
    /// Hakka (Sixian, Pha̍k-fa-sṳ)
    Hs,
    /// Hakka (Hailu, Taiwanese Hakka Romanization)
    Hh,
    /// Hakka (Meixian, Guangdong Romanization)
    Hm,
    /// Hakka (Changting, Changting Pinyin)
    Hc,
    /// Jin (Wiktionary)
    Jin,
    /// Min (All)
    Min,
    /// Northern Min (Gṳ̿ing-nǎing Lô̤-mǎ-cī)
    Minn,
    /// Eastern Min (Bàng-uâ-cê)
    Mine,
    /// Puxian Min (Pouseng Ping'ing)
    Minp,
    /// Southern Min (All)
    Mins,
    /// Hokkien (Pe̍h-ōe-jī)
    Minh,
    /// Teochew (Peng'im)
    Mint,
    /// Leizhou (Leizhou Pinyin)
    Minl,
    /// Southern Pinghua (Jyutping++)
    Sp,
    /// Wu (All)
    Wu,
    /// Wu (Northern, Wugniu)
    Wn,
    /// Wu (Jinhua, Wugniu)
    Wj,
    /// Xiang (All)
    Xiang,
    /// Xiang (Changsha, Wiktionary)
    Xc,
    /// Xiang (Loudi, Wiktionary)
    Xl,
    /// Xiang (Hengyang, Wiktionary)
    Xh,
}

#[derive(Debug, Default)]
pub struct Data {
    // Mandarin
    pub ma_standard: Option<String>,
    pub ma_chengdu: Option<String>,
    pub ma_xian: Option<String>,
    pub ma_nanjing: Option<String>,
    pub ma_dungan: Option<String>,
    // Cantonese
    pub ca_guangzhou: Option<String>,
    pub ca_dongguan: Option<String>,
    pub ca_taishan: Option<String>,
    pub ca_yangjiang: Option<String>,
    // Gan
    pub gan: Option<String>,
    // Hakka
    pub ha_sixian: Option<String>,
    pub ha_hailu: Option<String>,
    pub ha_meixian: Option<String>,
    pub ha_changting: Option<String>,
    // Jin
    pub jin: Option<String>,
    // Min
    pub mi_northern: Option<String>,
    pub mi_eastern: Option<String>,
    pub mi_puxian: Option<String>,
    pub mi_hokkien: Option<String>,
    pub mi_teochew: Option<String>,
    pub mi_leizhou: Option<String>,
    // Pinghua
    pub ph_southern: Option<String>,
    // Wu
    pub wu_northern: Option<String>,
    pub wu_jinhua: Option<String>,
    // Xiang
    pub xi_changsa: Option<String>,
    pub xi_loudi: Option<String>,
    pub xi_hengyang: Option<String>,
}

pub fn fetch(section: &str) -> Data {
    let re = &regexes().modern_data;

    let mut data = Data::default();

    if let Some(caps) = re.captures(section) {
        if let Some(data_str) = caps.get(1) {
            let wik_data: Value =
                serde_json::from_str(data_str.as_str()).expect("Invalid JSON data.");
            if let Some(prons) = get(&wik_data) {
                data.ma_standard = get_pron(prons, "m");
                data.ma_chengdu = get_pron(prons, "m-s");
                data.ma_xian = get_pron(prons, "m-x");
                data.ma_nanjing = get_pron(prons, "m-nj");
                data.ma_dungan = get_pron(prons, "dg");
                data.ca_guangzhou = get_pron(prons, "c");
                data.ca_dongguan = get_pron(prons, "c-dg");
                data.ca_taishan = get_pron(prons, "c-t");
                data.ca_yangjiang = get_pron(prons, "c-yj");
                data.gan = get_pron(prons, "g");
                if let Some(hakka) = get_pron(prons, "h") {
                    for mut sp in hakka.split(';').map(|s| s.split('=')) {
                        if let (Some(name), Some(lit)) = (sp.next(), sp.next()) {
                            let lit_raw = lit.to_string();
                            let lit_stripped = lit_raw.strip_prefix("h:").map(|s| s.to_string());
                            let lit = lit_stripped.or(Some(lit_raw));
                            match name {
                                "pfs" => data.ha_sixian = lit,
                                "hrs" => data.ha_hailu = lit,
                                "gd" => data.ha_meixian = lit,
                                "ct" => data.ha_changting = lit,
                                _ => {}
                            }
                        };
                    }
                }
                data.jin = get_pron(prons, "j");
                data.mi_northern = get_pron(prons, "mb");
                data.mi_eastern = get_pron(prons, "md");
                data.mi_hokkien = get_pron(prons, "mn").map(|h| simplify_romanizations(&h, '/'));
                data.mi_teochew = get_pron(prons, "mn-t");
                data.mi_leizhou = get_pron(prons, "mn-l");
                data.mi_puxian = get_pron(prons, "px").map(|p| simplify_romanizations(&p, '/'));
                data.ph_southern = get_pron(prons, "sp");
                data.wu_northern = get_pron(prons, "w").map(|w| simplify_romanizations(&w, ';'));
                data.wu_jinhua = get_pron(prons, "w-j");
                data.xi_changsa = get_pron(prons, "x");
                data.xi_loudi = get_pron(prons, "x-l");
                data.xi_hengyang = get_pron(prons, "x-h");
            }
        }
    }

    data
}

fn get(data: &Value) -> Option<&Value> {
    data.get("parts")?.get(0)?.get("template")?.get("params")
}

fn get_pron(data: &Value, name: &str) -> Option<String> {
    Some(data.get(name)?.get("wt")?.as_str()?.to_string())
}

fn simplify_romanizations(s: &str, sep: char) -> String {
    s.split(sep)
        .map(|s| s.split(':'))
        .filter_map(|mut s| {
            let (fst, snd) = (s.next(), s.next());
            snd.or(fst)
        })
        .collect::<Vec<_>>()
        .join("/")
}
