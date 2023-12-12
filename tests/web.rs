//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate mpkr;
use mpkr::{Mpkr, auffangstreitwert};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_auffangstreitwert() {
    assert_eq!(auffangstreitwert(), 5000);
}

#[wasm_bindgen_test]
pub fn test_geb_auffangstreitwert() {
    let mut mpkr = Mpkr::new();
    mpkr.set_streitwert(auffangstreitwert());
    assert_eq!(mpkr.gkg_geb(), 161);
    assert_eq!(mpkr.rvg13_geb(), 334);
    assert_eq!(mpkr.rvg49_geb(), 284);
}

#[wasm_bindgen_test]
pub fn test_einbergrg() {
    let mut mpkr = Mpkr::new();
    mpkr.set_thema(6);
    mpkr.set_anzahl(1);
    assert_eq!(mpkr.streitwert(), 10000);
    assert_eq!(mpkr.gkg_geb(), 266);
    assert_eq!(mpkr.rvg13_geb(), 614);
    assert_eq!(mpkr.rvg49_geb(), 339);
}