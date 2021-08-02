#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_color_string::*;

#[wasm_bindgen_test]
fn hsla_basic() {
    let expected = Some(Hsla {
        h: 240.0,
        s: 100.0,
        l: 50.5,
        a: 1.0,
    });
    assert_eq!(expected, get_hsla("hsl(240, 100%, 50.5%)"));
    assert_eq!(expected, get_hsla("hsl(240 100% 50.5%)"));
}
#[wasm_bindgen_test]
fn hsla_deg() {
    let expected = Some(Hsla {
        h: 240.0,
        s: 100.0,
        l: 50.5,
        a: 1.0,
    });
    assert_eq!(expected, get_hsla("hsl(240deg, 100%, 50.5%)"));
    assert_eq!(expected, get_hsla("hsl(240deg 100% 50.5%)"));
}

#[wasm_bindgen_test]
fn hsla_zero() {
    let expected = Some(Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 1.0,
    });
    assert_eq!(expected, get_hsla("hsla(0, 0%, 0%)"));
    assert_eq!(expected, get_hsla("hsla(0 0% 0%)"));
}

#[wasm_bindgen_test]
fn hsla_range() {
    let expected = Some(Hsla {
        h: 360.0,
        s: 10.0,
        l: 100.0,
        a: 1.0,
    });
    assert_eq!(expected, get_hsla("hsla(400, 10%, 200%)"));
    assert_eq!(expected, get_hsla("hsla(400 10% 200%)"));
}

#[wasm_bindgen_test]
fn hsla_operator_pos() {
    let expected = Some(Hsla {
        h: 240.0,
        s: 100.0,
        l: 50.5,
        a: 1.0,
    });
    assert_eq!(expected, get_hsla("hsl(+240, +100%, +50.5%)"));
    assert_eq!(expected, get_hsla("hsl(+240 +100% +50.5%)"));
}

#[wasm_bindgen_test]
fn hsla_operator_neg() {
    let expected = Some(Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 1.0,
    });
    assert_eq!(expected, get_hsla("hsl(-240, -100%, -50.5%)"));
    assert_eq!(expected, get_hsla("hsl(-240 -100% -50.5%)"));
}

#[wasm_bindgen_test]
fn hsla_alpha() {
    let expected = Some(Hsla {
        h: 240.0,
        s: 100.0,
        l: 50.5,
        a: 0.2,
    });
    assert_eq!(expected, get_hsla("hsla(240, 100%, 50.5%, 0.2)"));
    assert_eq!(expected, get_hsla("hsla(240 100% 50.5% / 0.2)"));
}

#[wasm_bindgen_test]
fn hsla_alpha_zero() {
    let expected = Some(Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 0.0,
    });
    assert_eq!(expected, get_hsla("hsla(0, 0%, 0%, 0)"));
    assert_eq!(expected, get_hsla("hsla(0 0% 0% / 0)"));
}

#[wasm_bindgen_test]
fn hsla_alpha_range() {
    let expected = Some(Hsla {
        h: 360.0,
        s: 10.0,
        l: 100.0,
        a: 1.0,
    });
    assert_eq!(expected, get_hsla("hsla(400, 10%, 200%, 10)"));
    assert_eq!(expected, get_hsla("hsla(400 10% 200% / 10)"));
}

#[wasm_bindgen_test]
fn hsla_alpha_pos() {
    let expected = Some(Hsla {
        h: 240.0,
        s: 100.0,
        l: 50.5,
        a: 0.8,
    });
    assert_eq!(expected, get_hsla("hsl(+240, +100%, +50.5%, +0.8)"));
    assert_eq!(expected, get_hsla("hsl(+240 +100% +50.5% / +0.8)"));
}

#[wasm_bindgen_test]
fn hsla_alpha_neg() {
    let expected = Some(Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 0.0,
    });
    assert_eq!(expected, get_hsla("hsl(-240, -100%, -50.5%, -1)"));
    assert_eq!(expected, get_hsla("hsl(-240 -100% -50.5% / -1)"));
}

#[wasm_bindgen_test]
fn hsla_combined() {
    let expected = Some(Hsla {
        h: 240.0,
        s: 100.0,
        l: 50.5,
        a: 0.0,
    });
    assert_eq!(expected, get_hsla("hsl(+240deg, 100%, 50.5%, -0.2)"));
    assert_eq!(expected, get_hsla("hsl(+240deg 100% 50.5% / -0.2)"));
}

#[wasm_bindgen_test]
fn hsla_alpha_percent() {
    assert_eq!(None, get_hsla("hsla(250, 100%, 50%, 50%)"));
    assert_eq!(None, get_hsla("hsla(250 100% 50% / 50%)"));
}

#[wasm_bindgen_test]
fn hsla_extra_at_start() {
    assert_eq!(None, get_hsla("1234hsl(41, 50%, 45%)"));
    assert_eq!(None, get_hsla("1234hsl(41 50% 45%)"));
    assert_eq!(None, get_hsla("1234hsl(41 50% 45% / 3)"));
}

#[wasm_bindgen_test]
fn hsla_extra_at_end() {
    assert_eq!(None, get_hsla("hsl(41, 50%, 45%)1234"));
    assert_eq!(None, get_hsla("hsl(41 50% 45%)1234"));
    assert_eq!(None, get_hsla("hsl(41 50% 45% / 3)1234"));
}
