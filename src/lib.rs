use esox::csv::load::niseci::{
    load_anagrafica_niseci_from_reader, load_campionamento_niseci_from_reader,
    load_riferimento_niseci_from_reader, AnagraficaNISECIError, CampionamentoNISECIError,
    RiferimentoNISECIError,
};
use esox::csv::load::hfbi::{
    load_campionamento_hfbi_from_reader, load_anagrafica_hfbi_from_reader, CampionamentoHFBIError, AnagraficaHFBIError
};
use esox::csv::deser::hfbi::{
    PlainRecordCsvAnagraficaHFBI,
    PlainRecordCsvCampionamentoHFBI, VeryItalianRecordCsvAnagraficaHFBI,
    VeryItalianRecordCsvCampionamentoHFBI,
};
use esox::csv::deser::niseci::{
    PlainRecordCsvAnagraficaNISECI,
    PlainRecordCsvCampionamentoNISECI, PlainRecordCsvRiferimentoNISECI,
    VeryItalianRecordCsvAnagraficaNISECI, VeryItalianRecordCsvCampionamentoNISECI,
    VeryItalianRecordCsvRiferimentoNISECI,
};
use esox::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI, RisultatoHFBI};
use esox::domain::niseci::{
    AnagraficaNISECI, CampionamentoNISECI, RiferimentoNISECI, RisultatoNISECI,
};
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
    let rif_vec;
    if is_italian {
        match load_riferimento_niseci_from_reader::<_, VeryItalianRecordCsvRiferimentoNISECI>(
            rif_reader,
            has_headers,
        ) {
            Ok(v) => rif_vec = v,
            Err(ev) => match ev {
                RiferimentoNISECIError::Csv(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
                RiferimentoNISECIError::Value(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
            },
        }
    } else {
        match load_riferimento_niseci_from_reader::<_, PlainRecordCsvRiferimentoNISECI>(
            rif_reader,
            has_headers,
        ) {
            Ok(v) => rif_vec = v,
            Err(ev) => match ev {
                RiferimentoNISECIError::Csv(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
                RiferimentoNISECIError::Value(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
            },
        }
    };
    let camp_reader = Cursor::new(camp_str.as_bytes());
    let camp_vec;
    if is_italian {
        match load_campionamento_niseci_from_reader::<_, VeryItalianRecordCsvCampionamentoNISECI>(
            camp_reader,
            has_headers,
            rif_vec.clone(),
        ) {
            Ok(v) => camp_vec = v,
            Err(ev) => match ev {
                CampionamentoNISECIError::Csv(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
                CampionamentoNISECIError::Value(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
            },
        }
    } else {
        match load_campionamento_niseci_from_reader::<_, PlainRecordCsvCampionamentoNISECI>(
            camp_reader,
            has_headers,
            rif_vec.clone(),
        ) {
            Ok(v) => camp_vec = v,
            Err(ev) => match ev {
                CampionamentoNISECIError::Csv(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
                CampionamentoNISECIError::Value(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
            },
        }
    }
    let anag_reader = Cursor::new(anag_str.as_bytes());
    let anag;
    if is_italian {
        match load_anagrafica_niseci_from_reader::<_, VeryItalianRecordCsvAnagraficaNISECI>(
            anag_reader,
            has_headers,
        ) {
            Ok(v) => anag = v,
            Err(ev) => match ev {
                AnagraficaNISECIError::Csv(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
                AnagraficaNISECIError::Value(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
            },
        }
    } else {
        match load_anagrafica_niseci_from_reader::<_, PlainRecordCsvAnagraficaNISECI>(
            anag_reader,
            has_headers,
        ) {
            Ok(v) => anag = v,
            Err(ev) => match ev {
                AnagraficaNISECIError::Csv(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
                AnagraficaNISECIError::Value(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
            },
        }
    }

    let riferimento = RiferimentoNISECI::new(rif_vec);
    let campionamento = CampionamentoNISECI::new(camp_vec);
    let anagrafica: AnagraficaNISECI = anag;
    match calculate_niseci(&campionamento, &riferimento, &anagrafica) {
        Ok(v) => {
            let rqe = calculate_rqe_niseci(v.0);
            let res = RisultatoNISECI::new(v.0, rqe, v.1);
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
    let camp_vec;
    if is_italian {
        match load_campionamento_hfbi_from_reader::<_, VeryItalianRecordCsvCampionamentoHFBI>(
            camp_reader,
            has_headers,
        ) {
            Ok(v) => camp_vec = v,
            Err(ev) => {
                match ev {
                    CampionamentoHFBIError::Csv(errors) => {
                        return Err(errors.into_iter().map(|e| e.to_string()).collect());
                    }
                    CampionamentoHFBIError::Value(errors) => {
                        return Err(errors.into_iter().map(|e| e.to_string()).collect());
                    }
                }
            }
        }
    } else {
        match load_campionamento_hfbi_from_reader::<_, PlainRecordCsvCampionamentoHFBI>(
            camp_reader,
            has_headers,
        ) {
            Ok(v) => camp_vec = v,
            Err(ev) => {
                match ev {
                    CampionamentoHFBIError::Csv(errors) => {
                        return Err(errors.into_iter().map(|e| e.to_string()).collect());
                    }
                    CampionamentoHFBIError::Value(errors) => {
                        return Err(errors.into_iter().map(|e| e.to_string()).collect());
                    }
                }
            }
        }
    }
    let anag_reader = Cursor::new(anag_str.as_bytes());
    let anag;
    if is_italian {
        match load_anagrafica_hfbi_from_reader::<_, VeryItalianRecordCsvAnagraficaHFBI>(
            anag_reader,
            has_headers,
        ) {
            Ok(v) => anag = v,
            Err(ev) => {
                match ev {
                    AnagraficaHFBIError::Csv(errors) => {
                        return Err(errors.into_iter().map(|e| e.to_string()).collect());
                    }
                    AnagraficaHFBIError::Value(errors) => {
                        return Err(errors.into_iter().map(|e| e.to_string()).collect());
                    }
                }
            }
        }
    } else {
        match load_anagrafica_hfbi_from_reader::<_, PlainRecordCsvAnagraficaHFBI>(
            anag_reader,
            has_headers,
        ) {
            Ok(v) => anag = v,
            Err(ev) => {
                match ev {
                    AnagraficaHFBIError::Csv(errors) => {
                        return Err(errors.into_iter().map(|e| e.to_string()).collect());
                    }
                    AnagraficaHFBIError::Value(errors) => {
                        return Err(errors.into_iter().map(|e| e.to_string()).collect());
                    }
                }
            }
        }
    }
    let campionamento = CampionamentoHFBI::new(camp_vec);
    let anagrafica: AnagraficaHFBI = anag;
    match calculate_hfbi(&campionamento, &anagrafica) {
        Ok(v) => {
            let res = RisultatoHFBI::new(Some(v.0), v.1);
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
