use esox::csv::deser::hfbi::{
    PlainRecordCsvAnagraficaHFBI, PlainRecordCsvCampionamentoHFBI,
    VeryItalianRecordCsvAnagraficaHFBI, VeryItalianRecordCsvCampionamentoHFBI,
    check_anagrafica_hfbi_reader, check_campionamento_hfbi_reader,
};
use esox::csv::deser::niseci::{
    PlainRecordCsvCampionamentoNISECI, PlainRecordCsvRiferimentoNISECI,
    PlainRecordCsvAnagraficaNISECI,
    VeryItalianRecordCsvAnagraficaNISECI, VeryItalianRecordCsvCampionamentoNISECI,
    VeryItalianRecordCsvRiferimentoNISECI, check_anagrafica_niseci_reader,
    check_campionamento_niseci_reader, check_riferimento_niseci_reader,
};
use esox::csv::parser::hfbi::{check_records_anagrafica_hfbi, check_records_campionamento_hfbi};
use esox::csv::parser::niseci::{
    check_records_anagrafica_niseci, check_records_campionamento_niseci,
    check_records_riferimento_niseci,
};
use esox::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI, RisultatoHFBI};
use esox::domain::niseci::{AnagraficaNISECI, CampionamentoNISECI, RiferimentoNISECI, RisultatoNISECI};
use esox::engines::hfbi::full::calculate_hfbi;
use esox::engines::niseci::full::{calculate_niseci, calculate_rqe_niseci};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calc_niseci_italian(
    rif_str: &str,
    camp_str: &str,
    anag_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let rif_reader = Cursor::new(rif_str.as_bytes());
    let checked_rif_vec;
    if is_italian {
        let rif_vec;
        match check_riferimento_niseci_reader::<_, VeryItalianRecordCsvRiferimentoNISECI>(
            rif_reader,
            has_headers,
        ) {
            Ok(v) => rif_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
        match check_records_riferimento_niseci(rif_vec) {
            Ok(v) => checked_rif_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
    } else {
        let rif_vec;
        match check_riferimento_niseci_reader::<_, PlainRecordCsvRiferimentoNISECI>(
            rif_reader,
            has_headers,
        ) {
            Ok(v) => rif_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
        match check_records_riferimento_niseci(rif_vec) {
            Ok(v) => checked_rif_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
    };
    let camp_reader = Cursor::new(camp_str.as_bytes());
    let checked_camp_vec;
    if is_italian {
        let camp_vec;
        match check_campionamento_niseci_reader::<_, VeryItalianRecordCsvCampionamentoNISECI>(
            camp_reader,
            has_headers,
        ) {
            Ok(v) => camp_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
        match check_records_campionamento_niseci(camp_vec, checked_rif_vec.clone()) {
            Ok(v) => checked_camp_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
    } else {
        let camp_vec;
        match check_campionamento_niseci_reader::<_, PlainRecordCsvCampionamentoNISECI>(
            camp_reader,
            has_headers,
        ) {
            Ok(v) => camp_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
        match check_records_campionamento_niseci(camp_vec, checked_rif_vec.clone()) {
            Ok(v) => checked_camp_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
    }
    let anag_reader = Cursor::new(anag_str.as_bytes());
    let checked_anag;
    if is_italian {
        let anag_vec;
        match check_anagrafica_niseci_reader::<_, VeryItalianRecordCsvAnagraficaNISECI>(
            anag_reader,
            has_headers,
        ) {
            Ok(v) => anag_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
        match check_records_anagrafica_niseci(anag_vec) {
            Ok(v) => checked_anag = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
    } else {
        let anag_vec;
        match check_anagrafica_niseci_reader::<_, PlainRecordCsvAnagraficaNISECI>(
            anag_reader,
            has_headers,
        ) {
            Ok(v) => anag_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
        match check_records_anagrafica_niseci(anag_vec) {
            Ok(v) => checked_anag = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
    }

    let riferimento = RiferimentoNISECI::new(checked_rif_vec);
    let campionamento = CampionamentoNISECI::new(checked_camp_vec);
    let anagrafica: AnagraficaNISECI = checked_anag;
    match calculate_niseci(&campionamento, &riferimento, &anagrafica) {
        Ok(v) => {
            let rqe = calculate_rqe_niseci(v.0);
            let res = RisultatoNISECI::new(
                v.0,
                rqe,
                v.1
            );
            match serde_wasm_bindgen::to_value(&res) {
                Ok(v) => Ok(v),
                Err(_) => Err(vec!["serde fail".to_string()]),
            }
        }
        Err(ev) => {
            return Err(ev);
        }
    }
}

#[wasm_bindgen]
pub fn calc_hfbi_italian(
    camp_str: &str,
    anag_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let camp_reader = Cursor::new(camp_str.as_bytes());
    let checked_camp_vec;
    if is_italian {
        let camp_vec;
        match check_campionamento_hfbi_reader::<_, VeryItalianRecordCsvCampionamentoHFBI>(
            camp_reader,
            has_headers,
        ) {
            Ok(v) => camp_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
        match check_records_campionamento_hfbi(camp_vec) {
            Ok(v) => checked_camp_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
    } else {
        let camp_vec;
        match check_campionamento_hfbi_reader::<_, PlainRecordCsvCampionamentoHFBI>(
            camp_reader,
            has_headers,
        ) {
            Ok(v) => camp_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
        match check_records_campionamento_hfbi(camp_vec) {
            Ok(v) => checked_camp_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
    }
    let anag_reader = Cursor::new(anag_str.as_bytes());
    let checked_anag;
    if is_italian {
        let anag_vec;
        match check_anagrafica_hfbi_reader::<_, VeryItalianRecordCsvAnagraficaHFBI>(
            anag_reader,
            has_headers,
        ) {
            Ok(v) => anag_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
        match check_records_anagrafica_hfbi(anag_vec) {
            Ok(v) => checked_anag = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
    } else {
        let anag_vec;
        match check_anagrafica_hfbi_reader::<_, PlainRecordCsvAnagraficaHFBI>(
            anag_reader,
            has_headers,
        ) {
            Ok(v) => anag_vec = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
        match check_records_anagrafica_hfbi(anag_vec) {
            Ok(v) => checked_anag = v,
            Err(ev) => {
                return Err(ev.into_iter().map(|e| e.to_string()).collect());
            }
        }
    }
    let campionamento = CampionamentoHFBI::new(checked_camp_vec);
    let anagrafica: AnagraficaHFBI = checked_anag;
    match calculate_hfbi(&campionamento, &anagrafica) {
        Ok(v) => {
            let res = RisultatoHFBI::new(
                Some(v.0),
                v.1,
            );
            match serde_wasm_bindgen::to_value(&res) {
                Ok(v) => Ok(v),
                Err(_) => Err(vec!["serde fail".to_string()]),
            }
        }
        Err(e) => {
            return Err(vec![e]);
        }
    }
}
