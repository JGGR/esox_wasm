// SPDX-License-Identifier: GPL-3.0-only
/*
    Copyright (C) 2024-2026 jgabaut, gioninjo

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, version 3 of the License.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

pub mod meta;

use esox::csv::deser::hfbi::{
    PlainRecordCsvAnagraficaHFBI, PlainRecordCsvCampionamentoHFBI,
    VeryItalianRecordCsvAnagraficaHFBI, VeryItalianRecordCsvCampionamentoHFBI,
};
use esox::csv::deser::niseci::{
    PlainRecordCsvAnagraficaNISECI, PlainRecordCsvCampionamentoNISECI,
    PlainRecordCsvRiferimentoNISECI, VeryItalianRecordCsvAnagraficaNISECI,
    VeryItalianRecordCsvCampionamentoNISECI, VeryItalianRecordCsvRiferimentoNISECI,
};
use esox::csv::load::hfbi::{
    load_anagrafica_hfbi_from_reader, load_campionamento_hfbi_from_reader, AnagraficaHFBIError,
    CampionamentoHFBIError,
};
use esox::csv::load::niseci::{
    load_anagrafica_niseci_from_reader, load_campionamento_niseci_from_reader,
    load_riferimento_niseci_from_reader, AnagraficaNISECIError, CampionamentoNISECIError,
    RiferimentoNISECIError,
};
use esox::domain::hfbi::{
    AnagraficaHFBI, CampionamentoHFBI, RisultatoHFBI, StatoEcologicoHFBI, ValoriIntermediHFBI,
};
use esox::domain::niseci::{
    AnagraficaNISECI, AreaNISECI, CampionamentoNISECI, RiferimentoNISECI, RisultatoNISECI,
    StatoEcologicoNISECI, ValoriIntermediNISECI,
};
use esox::engines::hfbi::full::calculate_hfbi;
use esox::engines::niseci::full::{calculate_niseci, calculate_rqe_niseci};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

fn load_riferimento_niseci(
    rif_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<RiferimentoNISECI, Vec<String>> {
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
    }
    Ok(RiferimentoNISECI::new(rif_vec))
}

#[wasm_bindgen]
pub fn parse_riferimento_niseci(
    rif_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let res = load_riferimento_niseci(rif_str, is_italian, has_headers);
    match serde_wasm_bindgen::to_value(&res) {
        Ok(v) => Ok(v),
        Err(_) => Err(vec!["serde fail".to_string()]),
    }
}

fn load_campionamento_niseci(
    camp_str: &str,
    riferimento: &RiferimentoNISECI,
    is_italian: bool,
    has_headers: bool,
) -> Result<CampionamentoNISECI, Vec<String>> {
    let camp_reader = Cursor::new(camp_str.as_bytes());
    let camp_vec;
    if is_italian {
        match load_campionamento_niseci_from_reader::<_, VeryItalianRecordCsvCampionamentoNISECI>(
            camp_reader,
            has_headers,
            riferimento.elenco_specie.clone(),
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
            riferimento.elenco_specie.clone(),
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
    Ok(CampionamentoNISECI::new(camp_vec))
}

#[wasm_bindgen]
pub fn parse_campionamento_niseci(
    camp_str: &str,
    riferimento: JsValue,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let riferimento: RiferimentoNISECI =
        serde_wasm_bindgen::from_value(riferimento).map_err(|e| vec![e.to_string()])?;
    let res = load_campionamento_niseci(camp_str, &riferimento, is_italian, has_headers);
    match serde_wasm_bindgen::to_value(&res) {
        Ok(v) => Ok(v),
        Err(_) => Err(vec!["serde fail".to_string()]),
    }
}

fn load_anagrafica_niseci(
    anag_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<AnagraficaNISECI, Vec<String>> {
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
    Ok(anag)
}

#[wasm_bindgen]
pub fn parse_anagrafica_niseci(
    anag_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let res = load_anagrafica_niseci(anag_str, is_italian, has_headers);
    match serde_wasm_bindgen::to_value(&res) {
        Ok(v) => Ok(v),
        Err(_) => Err(vec!["serde fail".to_string()]),
    }
}

fn calc_niseci_to_js(
    campionamento: &CampionamentoNISECI,
    riferimento: &RiferimentoNISECI,
    anagrafica: &AnagraficaNISECI,
) -> Result<JsValue, Vec<String>> {
    match calculate_niseci(campionamento, riferimento, anagrafica) {
        Ok(v) => {
            let rqe = calculate_rqe_niseci(v.0);
            let res = RisultatoNISECI::new(v.0, rqe, v.1);
            match serde_wasm_bindgen::to_value(&res) {
                Ok(v) => Ok(v),
                Err(_) => Err(vec!["serde fail".to_string()]),
            }
        }
        Err(ev) => Err(ev),
    }
}

#[wasm_bindgen]
pub fn calc_niseci_italian(
    rif_str: &str,
    camp_str: &str,
    anag_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let riferimento = load_riferimento_niseci(rif_str, is_italian, has_headers)?;
    let campionamento = load_campionamento_niseci(camp_str, &riferimento, is_italian, has_headers)?;
    let anagrafica = load_anagrafica_niseci(anag_str, is_italian, has_headers)?;
    calc_niseci_to_js(&campionamento, &riferimento, &anagrafica)
}

#[wasm_bindgen]
pub fn calc_niseci_from_js(
    riferimento: JsValue,
    campionamento: JsValue,
    anagrafica: JsValue,
) -> Result<JsValue, Vec<String>> {
    let riferimento: RiferimentoNISECI =
        serde_wasm_bindgen::from_value(riferimento).map_err(|e| vec![e.to_string()])?;

    let campionamento: CampionamentoNISECI =
        serde_wasm_bindgen::from_value(campionamento).map_err(|e| vec![e.to_string()])?;

    let anagrafica: AnagraficaNISECI =
        serde_wasm_bindgen::from_value(anagrafica).map_err(|e| vec![e.to_string()])?;
    calc_niseci_to_js(&campionamento, &riferimento, &anagrafica)
}

#[wasm_bindgen]
pub fn res_niseci_to_csv(
    res: JsValue,
    anagrafica: JsValue,
    comma_csv_delimiter: bool,
) -> Result<String, JsValue> {
    let anagrafica: AnagraficaNISECI = serde_wasm_bindgen::from_value(anagrafica)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let risultato: RisultatoNISECI =
        serde_wasm_bindgen::from_value(res).map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(risultato.to_csv(&anagrafica, comma_csv_delimiter))
}

#[wasm_bindgen]
pub fn intermediates_niseci_to_csv(
    intermediates: JsValue,
    comma_csv_delimiter: bool,
) -> Result<String, JsValue> {
    let intermediates: ValoriIntermediNISECI = serde_wasm_bindgen::from_value(intermediates)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(intermediates.to_csv(comma_csv_delimiter))
}

#[wasm_bindgen]
pub fn res_niseci_to_stato_eco_str(res: JsValue, area: JsValue) -> Result<String, JsValue> {
    let risultato: RisultatoNISECI =
        serde_wasm_bindgen::from_value(res).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let area: AreaNISECI =
        serde_wasm_bindgen::from_value(area).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(risultato
        .get_valore()
        .map(|v| StatoEcologicoNISECI::from((v, &area)))
        .ok_or("NC")?
        .to_string())
}

fn load_campionamento_hfbi(
    camp_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<CampionamentoHFBI, Vec<String>> {
    let camp_reader = Cursor::new(camp_str.as_bytes());
    let camp_vec;
    if is_italian {
        match load_campionamento_hfbi_from_reader::<_, VeryItalianRecordCsvCampionamentoHFBI>(
            camp_reader,
            has_headers,
        ) {
            Ok(v) => camp_vec = v,
            Err(ev) => match ev {
                CampionamentoHFBIError::Csv(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
                CampionamentoHFBIError::Value(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
            },
        }
    } else {
        match load_campionamento_hfbi_from_reader::<_, PlainRecordCsvCampionamentoHFBI>(
            camp_reader,
            has_headers,
        ) {
            Ok(v) => camp_vec = v,
            Err(ev) => match ev {
                CampionamentoHFBIError::Csv(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
                CampionamentoHFBIError::Value(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
            },
        }
    }
    Ok(CampionamentoHFBI::new(camp_vec))
}

#[wasm_bindgen]
pub fn parse_campionamento_hfbi(
    camp_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let res = load_campionamento_hfbi(camp_str, is_italian, has_headers);
    match serde_wasm_bindgen::to_value(&res) {
        Ok(v) => Ok(v),
        Err(_) => Err(vec!["serde fail".to_string()]),
    }
}

fn load_anagrafica_hfbi(
    anag_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<AnagraficaHFBI, Vec<String>> {
    let anag_reader = Cursor::new(anag_str.as_bytes());
    let anag;
    if is_italian {
        match load_anagrafica_hfbi_from_reader::<_, VeryItalianRecordCsvAnagraficaHFBI>(
            anag_reader,
            has_headers,
        ) {
            Ok(v) => anag = v,
            Err(ev) => match ev {
                AnagraficaHFBIError::Csv(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
                AnagraficaHFBIError::Value(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
            },
        }
    } else {
        match load_anagrafica_hfbi_from_reader::<_, PlainRecordCsvAnagraficaHFBI>(
            anag_reader,
            has_headers,
        ) {
            Ok(v) => anag = v,
            Err(ev) => match ev {
                AnagraficaHFBIError::Csv(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
                AnagraficaHFBIError::Value(errors) => {
                    return Err(errors.into_iter().map(|e| e.to_string()).collect());
                }
            },
        }
    }
    Ok(anag)
}

#[wasm_bindgen]
pub fn parse_anagrafica_hfbi(
    anag_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let res = load_anagrafica_hfbi(anag_str, is_italian, has_headers);
    match serde_wasm_bindgen::to_value(&res) {
        Ok(v) => Ok(v),
        Err(_) => Err(vec!["serde fail".to_string()]),
    }
}

fn calc_hfbi_to_js(
    campionamento: &CampionamentoHFBI,
    anagrafica: &AnagraficaHFBI,
) -> Result<JsValue, Vec<String>> {
    match calculate_hfbi(campionamento, anagrafica) {
        Ok(v) => {
            let res = RisultatoHFBI::new(Some(v.0), v.1);
            match serde_wasm_bindgen::to_value(&res) {
                Ok(v) => Ok(v),
                Err(_) => Err(vec!["serde fail".to_string()]),
            }
        }
        Err(e) => Err(vec![e]),
    }
}

#[wasm_bindgen]
pub fn calc_hfbi_italian(
    camp_str: &str,
    anag_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let campionamento = load_campionamento_hfbi(camp_str, is_italian, has_headers)?;
    let anagrafica = load_anagrafica_hfbi(anag_str, is_italian, has_headers)?;
    calc_hfbi_to_js(&campionamento, &anagrafica)
}

#[wasm_bindgen]
pub fn calc_hfbi_from_js(
    campionamento: JsValue,
    anagrafica: JsValue,
) -> Result<JsValue, Vec<String>> {
    let campionamento: CampionamentoHFBI =
        serde_wasm_bindgen::from_value(campionamento).map_err(|e| vec![e.to_string()])?;

    let anagrafica: AnagraficaHFBI =
        serde_wasm_bindgen::from_value(anagrafica).map_err(|e| vec![e.to_string()])?;
    calc_hfbi_to_js(&campionamento, &anagrafica)
}

#[wasm_bindgen]
pub fn res_hfbi_to_csv(
    res: JsValue,
    anagrafica: JsValue,
    comma_csv_delimiter: bool,
) -> Result<String, JsValue> {
    let anagrafica: AnagraficaHFBI = serde_wasm_bindgen::from_value(anagrafica)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let risultato: RisultatoHFBI =
        serde_wasm_bindgen::from_value(res).map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(risultato.to_csv(&anagrafica, comma_csv_delimiter))
}

#[wasm_bindgen]
pub fn intermediates_hfbi_to_csv(
    intermediates: JsValue,
    comma_csv_delimiter: bool,
) -> Result<String, JsValue> {
    let intermediates: ValoriIntermediHFBI = serde_wasm_bindgen::from_value(intermediates)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(intermediates.to_csv(comma_csv_delimiter))
}

#[wasm_bindgen]
pub fn res_hfbi_to_stato_eco_str(res: JsValue) -> Result<String, JsValue> {
    let risultato: RisultatoHFBI =
        serde_wasm_bindgen::from_value(res).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(risultato
        .get_valore()
        .map(StatoEcologicoHFBI::from)
        .ok_or("NC")?
        .to_string())
}

#[wasm_bindgen]
pub fn get_version() -> String {
    use crate::meta::version;
    version().to_string()
}

#[wasm_bindgen]
pub fn get_esox_version() -> String {
    use esox::meta::version;
    version().to_string()
}
