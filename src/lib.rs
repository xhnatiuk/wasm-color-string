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
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

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
pub fn get_rgb(string: &str) -> Option<Rgb> {
    lazy_static! {
        static ref RE_HEX_SHORT: Regex = Regex::new(r"^#([0-9A-Fa-f]{3})([0-9A-Fa-f]{1})?$").unwrap();
        static ref RE_HEX_LONG: Regex = Regex::new(r"^#([0-9A-Fa-f]{6})([0-9A-Fa-f]{2})?$").unwrap();
        static ref RE_RGB: Regex = Regex::new(r"^rgba?\(\s*([+-]?\d+)\s*,?\s*([+-]?\d+)\s*,?\s*([+-]?\d+)\s*(?:[,/]\s*([+-]?\d*(?:\.\d+)?%?)\s*)?\)$").unwrap();
        static ref RE_RGB_PERCENT: Regex = Regex::new(r"^rgba?\(\s*([+-]?[\d\.]+)%\s*,?\s*([+-]?[\d\.]+)%\s*,?\s*([+-]?[\d\.]+)%\s*(?:[,/]\s*([+-]?\d*(?:\.\d+)?%?)\s*)?\)$").unwrap();
    }
    if RE_HEX_SHORT.is_match(string) {
        let groups = RE_HEX_SHORT.captures(string)?;
        let rgb = groups.get(1)?.as_str();
        let r: u8 = parse_u8_hex(&rgb[0..1], 0, 15)?;
        let g: u8 = parse_u8_hex(&rgb[1..2], 0, 15)?;
        let b: u8 = parse_u8_hex(&rgb[2..3], 0, 15)?;
        let a: f32 = match groups.get(2) {
            Some(value) => {
                let alpha = parse_u8_hex(value.as_str(), 0, 15)?;
                ((16 * alpha) + alpha) as f32 / 255.0
            }
            None => 1.0,
        };
        return Some(Rgb {
            r: (16 * r) + r,
            g: (16 * g) + g,
            b: (16 * b) + b,
            a,
        });
    } else if RE_HEX_LONG.is_match(string) {
        let groups = RE_HEX_LONG.captures(string)?;
        let rgb = groups.get(1)?.as_str();
        let r: u8 = parse_u8_hex(&rgb[0..2], 0, 255)?;
        let g: u8 = parse_u8_hex(&rgb[2..4], 0, 255)?;
        let b: u8 = parse_u8_hex(&rgb[4..6], 0, 255)?;
        let a: f32 = match groups.get(2) {
            Some(value) => parse_u8_hex(value.as_str(), 0, 255)? as f32 / 255.0,
            None => 1.0,
        };
        return Some(Rgb { r, g, b, a });
    } else if RE_RGB.is_match(string) {
        let groups = RE_RGB.captures(string)?;
        let r: u8 = parse_u8(groups.get(1)?.as_str(), 0, 255)?;
        let g: u8 = parse_u8(groups.get(2)?.as_str(), 0, 255)?;
        let b: u8 = parse_u8(groups.get(3)?.as_str(), 0, 255)?;
        let a: f32 = get_alpha(groups.get(4))?;
        return Some(Rgb { r, g, b, a });
    } else if RE_RGB_PERCENT.is_match(string) {
        let groups = RE_RGB_PERCENT.captures(string)?;
        let r: u8 = (get_percentage(groups.get(1)?.as_str())? * 2.55) as u8;
        let g: u8 = (get_percentage(groups.get(1)?.as_str())? * 2.55) as u8;
        let b: u8 = (get_percentage(groups.get(1)?.as_str())? * 2.55) as u8;
        let a: f32 = get_alpha(groups.get(4))?;
        return Some(Rgb { r, g, b, a });
    } else {
        return None;
    }
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

fn parse_u8(string: &str, min: u8, max: u8) -> Option<u8> {
    Some(string.parse::<u8>().ok()?.max(min).min(max))
}

fn parse_u8_hex(string: &str, min: u8, max: u8) -> Option<u8> {
    Some(u8::from_str_radix(string, 16).ok()?.max(min).min(max))
}
