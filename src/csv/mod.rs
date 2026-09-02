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
use esox::csv::load::hfbi::{
    load_anagrafica_hfbi_from_reader, load_campionamento_hfbi_from_reader, AnagraficaHFBIError,
    CampionamentoHFBIError,
};
use esox::csv::load::niseci::{
    load_anagrafica_niseci_from_reader, load_campionamento_niseci_from_reader,
    load_riferimento_niseci_from_reader, AnagraficaNISECIError, CampionamentoNISECIError,
    RiferimentoNISECIError,
};
use esox::csv::load::InputFormat;
use esox::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI};
use esox::domain::niseci::{AnagraficaNISECI, CampionamentoNISECI, RiferimentoNISECI};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

pub(crate) fn load_csv_riferimento_niseci(
    rif_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<RiferimentoNISECI, Vec<String>> {
    let rif_reader = Cursor::new(rif_str.as_bytes());
    let format = if is_italian {
        InputFormat::Alternative
    } else {
        InputFormat::Standard
    };
    load_riferimento_niseci_from_reader::<_>(rif_reader, has_headers, format).map_err(|ev| match ev
    {
        RiferimentoNISECIError::Csv(errors) => errors.into_iter().map(|e| e.to_string()).collect(),
        RiferimentoNISECIError::Value(errors) => {
            errors.into_iter().map(|e| e.to_string()).collect()
        }
    })
}

#[wasm_bindgen]
#[cfg(feature = "experimental")]
pub fn parse_csv_riferimento_niseci(
    rif_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let res = load_csv_riferimento_niseci(rif_str, is_italian, has_headers);
    match serde_wasm_bindgen::to_value(&res) {
        Ok(v) => Ok(v),
        Err(_) => Err(vec!["serde fail".to_string()]),
    }
}

pub(crate) fn load_csv_campionamento_niseci(
    camp_str: &str,
    riferimento: &RiferimentoNISECI,
    is_italian: bool,
    has_headers: bool,
) -> Result<CampionamentoNISECI, Vec<String>> {
    let camp_reader = Cursor::new(camp_str.as_bytes());
    let format = if is_italian {
        InputFormat::Alternative
    } else {
        InputFormat::Standard
    };
    load_campionamento_niseci_from_reader::<_>(camp_reader, has_headers, riferimento, format)
        .map_err(|ev| match ev {
            CampionamentoNISECIError::Csv(errors) => {
                errors.into_iter().map(|e| e.to_string()).collect()
            }
            CampionamentoNISECIError::Value(errors) => {
                errors.into_iter().map(|e| e.to_string()).collect()
            }
        })
}

#[wasm_bindgen]
#[cfg(feature = "experimental")]
pub fn parse_csv_campionamento_niseci(
    camp_str: &str,
    riferimento: JsValue,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let riferimento: RiferimentoNISECI =
        serde_wasm_bindgen::from_value(riferimento).map_err(|e| vec![e.to_string()])?;
    let res = load_csv_campionamento_niseci(camp_str, &riferimento, is_italian, has_headers);
    match serde_wasm_bindgen::to_value(&res) {
        Ok(v) => Ok(v),
        Err(_) => Err(vec!["serde fail".to_string()]),
    }
}

pub(crate) fn load_csv_anagrafica_niseci(
    anag_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<AnagraficaNISECI, Vec<String>> {
    let anag_reader = Cursor::new(anag_str.as_bytes());
    let format = if is_italian {
        InputFormat::Alternative
    } else {
        InputFormat::Standard
    };
    load_anagrafica_niseci_from_reader::<_>(anag_reader, has_headers, format).map_err(|ev| match ev
    {
        AnagraficaNISECIError::Csv(errors) => errors.into_iter().map(|e| e.to_string()).collect(),
        AnagraficaNISECIError::Value(errors) => errors.into_iter().map(|e| e.to_string()).collect(),
    })
}

#[wasm_bindgen]
#[cfg(feature = "experimental")]
pub fn parse_csv_anagrafica_niseci(
    anag_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let res = load_csv_anagrafica_niseci(anag_str, is_italian, has_headers);
    match serde_wasm_bindgen::to_value(&res) {
        Ok(v) => Ok(v),
        Err(_) => Err(vec!["serde fail".to_string()]),
    }
}

pub(crate) fn load_csv_campionamento_hfbi(
    camp_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<CampionamentoHFBI, Vec<String>> {
    let camp_reader = Cursor::new(camp_str.as_bytes());
    let format = if is_italian {
        InputFormat::Alternative
    } else {
        InputFormat::Standard
    };
    load_campionamento_hfbi_from_reader::<_>(camp_reader, has_headers, format).map_err(
        |ev| match ev {
            CampionamentoHFBIError::Csv(errors) => {
                errors.into_iter().map(|e| e.to_string()).collect()
            }
            CampionamentoHFBIError::Value(errors) => {
                errors.into_iter().map(|e| e.to_string()).collect()
            }
        },
    )
}

#[wasm_bindgen]
pub fn parse_csv_campionamento_hfbi(
    camp_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let res = load_csv_campionamento_hfbi(camp_str, is_italian, has_headers);
    match serde_wasm_bindgen::to_value(&res) {
        Ok(v) => Ok(v),
        Err(_) => Err(vec!["serde fail".to_string()]),
    }
}

pub(crate) fn load_csv_anagrafica_hfbi(
    anag_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<AnagraficaHFBI, Vec<String>> {
    let anag_reader = Cursor::new(anag_str.as_bytes());
    let format = if is_italian {
        InputFormat::Alternative
    } else {
        InputFormat::Standard
    };
    load_anagrafica_hfbi_from_reader::<_>(anag_reader, has_headers, format).map_err(|ev| match ev {
        AnagraficaHFBIError::Csv(errors) => errors.into_iter().map(|e| e.to_string()).collect(),
        AnagraficaHFBIError::Value(errors) => errors.into_iter().map(|e| e.to_string()).collect(),
    })
}

#[wasm_bindgen]
pub fn parse_csv_anagrafica_hfbi(
    anag_str: &str,
    is_italian: bool,
    has_headers: bool,
) -> Result<JsValue, Vec<String>> {
    let res = load_csv_anagrafica_hfbi(anag_str, is_italian, has_headers);
    match serde_wasm_bindgen::to_value(&res) {
        Ok(v) => Ok(v),
        Err(_) => Err(vec!["serde fail".to_string()]),
    }
}
