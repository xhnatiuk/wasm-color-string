#[macro_use]
extern crate lazy_static;

use regex::{Match, Regex};
use wasm_bindgen::prelude::*;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
    pub a: f32,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Hwb {
    pub h: f32,
    pub w: f32,
    pub b: f32,
    pub a: f32,
}

#[wasm_bindgen]
pub fn get_hsl(string: &str) -> Option<Hsl> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^hsla?\(\s*([+-]?\d{0,3}(?:\.\d+)?)(?:deg)?\s*,?\s*([+-]?[\d\.]+)%\s*,?\s*([+-]?[\d\.]+)%\s*(?:[,/]\s*([+-]?\d*(?:\.\d+)?%?)\s*)?\)$").unwrap();
    }
    let groups = RE.captures(string)?;
    let h: f32 = groups
        .get(1)?
        .as_str()
        .parse::<f32>()
        .ok()?
        .rem_euclid(360.0);
    let s: f32 = get_percentage(groups.get(2)?.as_str())?;
    let l: f32 = get_percentage(groups.get(3)?.as_str())?;
    let a: f32 = get_alpha(groups.get(4))?;
    Some(Hsl { h, s, l, a })
}

#[wasm_bindgen]
pub fn get_hwb(string: &str) -> Option<Hwb> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^hwb\(\s*([+-]?\d{0,3}(?:\.\d+)?)(?:deg)?\s*,?\s*([+-]?[\d\.]+)%\s*,?\s*([+-]?[\d\.]+)%\s*(?:[,/]\s*([+-]?\d*(?:\.\d+)?%?)\s*)?\)$").unwrap();
    }
    let groups = RE.captures(string)?;
    let h: f32 = groups
        .get(1)?
        .as_str()
        .parse::<f32>()
        .ok()?
        .rem_euclid(360.0);
    let w: f32 = get_percentage(groups.get(2)?.as_str())?;
    let b: f32 = get_percentage(groups.get(3)?.as_str())?;
    let a: f32 = get_alpha(groups.get(4))?;
    Some(Hwb { h, w, b, a })
}

fn get_alpha(option: Option<Match>) -> Option<f32> {
    match option {
        Some(value) => parse_alpha(value.as_str()),
        None => Some(1.0),
    }
}

fn parse_alpha(string: &str) -> Option<f32> {
    let len = string.len();
    let last_char = string.chars().last().unwrap();
    if last_char == '%' {
        Some(get_percentage(&string[0..len - 1])? / 100.0)
    } else {
        parse_f32(&string, 0.0, 1.0)
    }
}

fn get_percentage(string: &str) -> Option<f32> {
    parse_f32(&string, 0.0, 100.0)
}

fn parse_f32(string: &str, min: f32, max: f32) -> Option<f32> {
    Some(string.parse::<f32>().ok()?.max(min).min(max))
}
