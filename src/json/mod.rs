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
use esox::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI};
use esox::domain::niseci::{AnagraficaNISECI, CampionamentoNISECI, RiferimentoNISECI};
use esox::deser::{PlainRecordAnagraficaHFBI, PlainRecordCampionamentoHFBI,
    PlainRecordAnagraficaNISECI, PlainRecordCampionamentoNISECI,
    PlainRecordRiferimentoNISECI,
};
use esox::json::load::hfbi::{
    load_json_anagrafica_hfbi_from_reader, load_json_campionamento_hfbi_from_reader,
    AnagraficaHFBIError, CampionamentoHFBIError,
};
use esox::json::load::niseci::{
    load_json_anagrafica_niseci_from_reader, load_json_campionamento_niseci_from_reader,
    load_json_riferimento_niseci_from_reader, AnagraficaNISECIError, CampionamentoNISECIError,
    RiferimentoNISECIError,
};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calc_niseci_json(
    rif_str: &str,
    camp_str: &str,
    anag_str: &str,
) -> Result<JsValue, Vec<String>> {
    let riferimento = load_json_riferimento_niseci(rif_str)?;
    let campionamento = load_json_campionamento_niseci(camp_str, &riferimento)?;
    let anagrafica = load_json_anagrafica_niseci(anag_str)?;
    calc_niseci_to_js(&campionamento, &riferimento, &anagrafica)
}

#[wasm_bindgen]
pub fn calc_hfbi_json(camp_str: &str, anag_str: &str) -> Result<JsValue, Vec<String>> {
    let campionamento = load_json_campionamento_hfbi(camp_str)?;
    let anagrafica = load_json_anagrafica_hfbi(anag_str)?;
    calc_hfbi_to_js(&campionamento, &anagrafica)
}

pub(crate) fn load_json_riferimento_niseci(
    rif_str: &str,
) -> Result<RiferimentoNISECI, Vec<String>> {
    let rif_reader = Cursor::new(rif_str.as_bytes());
    load_json_riferimento_niseci_from_reader::<_, PlainRecordRiferimentoNISECI>(rif_reader)
        .map_err(|ev| match ev {
            RiferimentoNISECIError::Json(errors) => {
                errors.into_iter().map(|e| e.to_string()).collect()
            }
            RiferimentoNISECIError::Value(errors) => {
                errors.into_iter().map(|e| e.to_string()).collect()
            }
            RiferimentoNISECIError::Io(e) => vec![e.to_string()],
        })
}

pub(crate) fn load_json_campionamento_niseci(
    camp_str: &str,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, Vec<String>> {
    let camp_reader = Cursor::new(camp_str.as_bytes());
    load_json_campionamento_niseci_from_reader::<_, PlainRecordCampionamentoNISECI>(
        camp_reader,
        riferimento,
    )
    .map_err(|ev| match ev {
        CampionamentoNISECIError::Json(errors) => {
            errors.into_iter().map(|e| e.to_string()).collect()
        }
        CampionamentoNISECIError::Value(errors) => {
            errors.into_iter().map(|e| e.to_string()).collect()
        }
        CampionamentoNISECIError::Io(e) => vec![e.to_string()],
    })
}

pub(crate) fn load_json_anagrafica_niseci(anag_str: &str) -> Result<AnagraficaNISECI, Vec<String>> {
    let anag_reader = Cursor::new(anag_str.as_bytes());
    load_json_anagrafica_niseci_from_reader::<_, PlainRecordAnagraficaNISECI>(anag_reader)
        .map_err(|ev| match ev {
            AnagraficaNISECIError::Json(errors) => {
                errors.into_iter().map(|e| e.to_string()).collect()
            }
            AnagraficaNISECIError::Value(errors) => {
                errors.into_iter().map(|e| e.to_string()).collect()
            }
            AnagraficaNISECIError::Io(e) => vec![e.to_string()],
        })
}

pub(crate) fn load_json_campionamento_hfbi(
    camp_str: &str,
) -> Result<CampionamentoHFBI, Vec<String>> {
    let camp_reader = Cursor::new(camp_str.as_bytes());
    load_json_campionamento_hfbi_from_reader::<_, PlainRecordCampionamentoHFBI>(camp_reader)
        .map_err(|ev| match ev {
            CampionamentoHFBIError::Json(errors) => {
                errors.into_iter().map(|e| e.to_string()).collect()
            }
            CampionamentoHFBIError::Value(errors) => {
                errors.into_iter().map(|e| e.to_string()).collect()
            }
            CampionamentoHFBIError::Io(e) => vec![e.to_string()],
        })
}

pub(crate) fn load_json_anagrafica_hfbi(anag_str: &str) -> Result<AnagraficaHFBI, Vec<String>> {
    let anag_reader = Cursor::new(anag_str.as_bytes());
    load_json_anagrafica_hfbi_from_reader::<_, PlainRecordAnagraficaHFBI>(anag_reader).map_err(
        |ev| match ev {
            AnagraficaHFBIError::Json(errors) => {
                errors.into_iter().map(|e| e.to_string()).collect()
            }
            AnagraficaHFBIError::Value(errors) => {
                errors.into_iter().map(|e| e.to_string()).collect()
            }
            AnagraficaHFBIError::Io(e) => vec![e.to_string()],
        },
    )
}
