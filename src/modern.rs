use crate::regexes::regexes;
use serde_json::{self, Value};

#[derive(Debug, Default)]
pub struct Data {
    // Mandarin
    ma_standard: Option<String>,
    ma_chengdu: Option<String>,
    ma_xian: Option<String>,
    ma_nanjing: Option<String>,
    ma_dungan: Option<String>,
    // Cantonese
    ca_guangzhou: Option<String>,
    ca_dongguan: Option<String>,
    ca_taishan: Option<String>,
    ca_yangjiang: Option<String>,
    // Gan
    gan: Option<String>,
    // Hakka
    ha_sixian: Option<String>,
    ha_hailu: Option<String>,
    ha_meixian: Option<String>,
    ha_changting: Option<String>,
    // Jin
    jin: Option<String>,
    // Min
    mi_northern: Option<String>,
    mi_eastern: Option<String>,
    mi_puxian: Option<String>,
    mi_hokkien: Option<String>,
    mi_teochew: Option<String>,
    mi_leizhou: Option<String>,
    // Pinghua
    ph_southern: Option<String>,
    // Wu
    wu_northern: Option<String>,
    wu_jinhua: Option<String>,
    // Xiang
    xi_changsa: Option<String>,
    xi_loudi: Option<String>,
    xi_hengyang: Option<String>,
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
