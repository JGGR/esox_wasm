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
use crate::{calc_hfbi_to_js, calc_niseci_to_js};
use esox::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI, RisultatoHFBI};
use esox::domain::niseci::{
    AnagraficaNISECI, CampionamentoNISECI, RiferimentoNISECI, RisultatoNISECI,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[cfg(feature = "experimental")]
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
#[cfg(feature = "experimental")]
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
#[cfg(feature = "experimental")]
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
#[cfg(feature = "experimental")]
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
