mod utils;
use regex::Regex;
use wasm_bindgen::prelude::*;

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
pub fn get_hsl(string: &str) -> Option<Hsl> {
    let re = Regex::new(r"^hsla?\(\s*([+-]?(?:\d{0,3}\.)?\d+)(?:deg)?\s*,?\s*([+-]?[\d\.]+)%\s*,?\s*([+-]?[\d\.]+)%\s*(?:[,|/]\s*([+-]?[\d\.]+)\s*)?\)$").unwrap();
    let captures = re.captures(string)?;
    let h: f32 = match captures.get(1)?.as_str().parse::<f32>() {
        Ok(num) => clamp(num, 0.0, 360.0),
        Err(_e) => return None,
    };
    let s: f32 = match captures.get(2)?.as_str().parse::<f32>() {
        Ok(num) => clamp(num, 0.0, 100.0),
        Err(_e) => return None,
    };
    let l: f32 = match captures.get(3)?.as_str().parse::<f32>() {
        Ok(num) => clamp(num, 0.0, 100.0),
        Err(_e) => return None,
    };
    let a: f32 = match captures.get(4) {
        Some(value) => match value.as_str().parse::<f32>() {
            Ok(num) => clamp(num, 0.0, 1.0),
            Err(_e) => return None,
        },
        None => 1.0,
    };
    Some(Hsl { h, s, l, a })
}

fn clamp(num: f32, min: f32, max: f32) -> f32 {
    num.max(min).min(max)
}
