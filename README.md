# esox_wasm

## WASM glue library for NISECI and HFBI calc

## Table of Contents

+ [What is this thing?](#witt)
+ [Input templates](#input_templates)
+ [Locale](#locale)
+ [Building](#building)
+ [Testing](#testing)
+ [References](#references)

## What is this thing? <a name = "witt"></a>

This is a WASM glue library for calculating the NISECI and/or HFBI index for a dataset.

## Input templates <a name = "input_templates"></a>

You can find templates for the input files in the `templates` folder.

## Locale <a name = "locale"></a>

Since this library is built with knowing that Excel uses some specific separators with the Italian locale, it supports two formats for input/output.

The format differences are:
- Italian:
  - Input expects `;` as csv field delimiter, and `,` can be used as float decimal delimiter
  - Output uses `;` as csv field delimiter, and floats are printed with `,` as decimal delimiter
- International:
  - Input expectes `,` as csv field delimiter, and `.` as float decimal delimiter
  - Output uses `,` as csv field delimiter, and floats are printed with `.` as decimal delimiter

## Building <a name = "building"></a>

To build the `pkg/.wasm` file, run:
```sh
    cargo install wasm-pack
    wasm-pack build --target web`
```

## Testing <a name = "testing"></a>

To test `pkg/.wasm` file, run:
```sh
    python -m http.server
```
This should start a server on `http://0.0.0.0:8000` using the provided `index.hmtl`.

## References <a name = "references"></a>

- [ISPRA - Nuove indice dello stato ecologico delle comunità ittiche - NISECI](https://www.isprambiente.gov.it/it/pubblicazioni/manuali-e-linee-guida/nuovo-indice-dello-stato-ecologico-delle-comunita-ittiche-niseci)
- [ISPRA - Manuale per la classificazione dell'Elemento di Qualità Biologica "Fauna Ittica" nelle lagune costiere italiane - HFBI](https://www.isprambiente.gov.it/it/pubblicazioni/manuali-e-linee-guida/manuale-per-la-classificazione-dell-elemento-di-qualita-biologica-fauna-ittica-nelle-lagune-costiere-italiane)
