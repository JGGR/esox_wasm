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
pub(crate) mod csv;
#[cfg(feature = "experimental")]
pub mod exper;
#[cfg(feature = "json")]
pub mod json;
pub mod meta;
use crate::csv::{
    load_csv_anagrafica_hfbi, load_csv_anagrafica_niseci, load_csv_campionamento_hfbi,
    load_csv_campionamento_niseci, load_csv_riferimento_niseci,
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
use wasm_bindgen::prelude::*;

pub(crate) fn calc_niseci_to_js(
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
    let riferimento = load_csv_riferimento_niseci(rif_str, is_italian, has_headers)?;
    let campionamento =
        load_csv_campionamento_niseci(camp_str, &riferimento, is_italian, has_headers)?;
    let anagrafica = load_csv_anagrafica_niseci(anag_str, is_italian, has_headers)?;
    calc_niseci_to_js(&campionamento, &riferimento, &anagrafica)
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

pub(crate) fn calc_hfbi_to_js(
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
    let campionamento = load_csv_campionamento_hfbi(camp_str, is_italian, has_headers)?;
    let anagrafica = load_csv_anagrafica_hfbi(anag_str, is_italian, has_headers)?;
    calc_hfbi_to_js(&campionamento, &anagrafica)
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
