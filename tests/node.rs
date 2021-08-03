#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_color_string::*;

#[wasm_bindgen_test]
fn color() {
    let expected_rgb = Some(Color(Model::Rgb, 204.0, 204.0, 204.0, 1.0));
    let expected_hsl = Some(Color(Model::Hsl, 240.0, 100.0, 50.5, 1.0));
    let expected_hwb = Some(Color(Model::Hwb, 240.0, 100.0, 50.5, 1.0));

    assert_eq!(expected_rgb, get_color("#CCC"));
    assert_eq!(expected_rgb, get_color("#CCCCCC"));
    assert_eq!(expected_rgb, get_color("rgb(204, 204, 204)"));
    assert_eq!(expected_rgb, get_color("rgb(204 204 204)"));
    assert_eq!(expected_rgb, get_color("rgb(80%, 80%, 80%)"));
    assert_eq!(expected_rgb, get_color("rgb(80% 80% 80%)"));

    assert_eq!(expected_hsl, get_color("hsl(240, 100%, 50.5%)"));
    assert_eq!(expected_hsl, get_color("hsl(240 100% 50.5%)"));
    assert_eq!(expected_hsl, get_color("hsla(240deg, 100%, 50.5%)"));
    assert_eq!(expected_hsl, get_color("hsla(240deg 100% 50.5%)"));

    assert_eq!(expected_hwb, get_color("hwb(240, 100%, 50.5%)"));
    assert_eq!(expected_hwb, get_color("hwb(240 100% 50.5%)"));
    assert_eq!(expected_hwb, get_color("hwb(240deg, 100%, 50.5%)"));
    assert_eq!(expected_hwb, get_color("hwb(240deg 100% 50.5%)"));
}

#[wasm_bindgen_test]
fn rgb_basic() {
    let expected = Some(Rgb {
        r: 204,
        g: 204,
        b: 204,
        a: 1.0,
    });
    assert_eq!(expected, get_rgb("#CCC"));
    assert_eq!(expected, get_rgb("#CCCCCC"));
    assert_eq!(expected, get_rgb("rgb(204, 204, 204)"));
    assert_eq!(expected, get_rgb("rgb(204 204 204)"));
    assert_eq!(expected, get_rgb("rgb(80%, 80%, 80%)"));
    assert_eq!(expected, get_rgb("rgb(80% 80% 80%)"));
}

#[wasm_bindgen_test]
fn rgb_zero() {
    let expected = Some(Rgb {
        r: 0,
        g: 0,
        b: 0,
        a: 1.0,
    });
    assert_eq!(expected, get_rgb("#000"));
    assert_eq!(expected, get_rgb("#000000"));
    assert_eq!(expected, get_rgb("rgb(0, 0, 0)"));
    assert_eq!(expected, get_rgb("rgb(0 0 0)"));
    assert_eq!(expected, get_rgb("rgb(0%, 0%, 0%)"));
    assert_eq!(expected, get_rgb("rgb(0% 0% 0%)"));
}

#[wasm_bindgen_test]
fn rgb_range() {
    let expected = Some(Rgb {
        r: 255,
        g: 255,
        b: 255,
        a: 1.0,
    });
    assert_eq!(None, get_rgb("#GGG"));
    assert_eq!(None, get_rgb("#GGGGGG"));
    assert_eq!(None, get_rgb("rgb(510, 510, 510)"));
    assert_eq!(None, get_rgb("rgb(510 510 510)"));
    assert_eq!(expected, get_rgb("rgb(200%, 200%, 200%)"));
    assert_eq!(expected, get_rgb("rgb(200% 200% 200%)"));
}

#[wasm_bindgen_test]
fn rgb_pos() {
    let expected = Some(Rgb {
        r: 204,
        g: 204,
        b: 204,
        a: 1.0,
    });
    assert_eq!(expected, get_rgb("rgb(+204, +204, +204)"));
    assert_eq!(expected, get_rgb("rgb(+204 +204 +204)"));
    assert_eq!(expected, get_rgb("rgb(+80%, +80%, +80%)"));
    assert_eq!(expected, get_rgb("rgb(+80% +80% +80%)"));
}

#[wasm_bindgen_test]
fn rgb_neg() {
    let expected = Some(Rgb {
        r: 0,
        g: 0,
        b: 0,
        a: 1.0,
    });
    assert_eq!(None, get_rgb("rgb(-204, -204, -204)"));
    assert_eq!(None, get_rgb("rgb(-204 -204 -204)"));
    assert_eq!(expected, get_rgb("rgb(-80%, -80%, -80%)"));
    assert_eq!(expected, get_rgb("rgb(-80% -80% -80%)"));
}

#[wasm_bindgen_test]
fn rgb_alpha_basic() {
    let expected = Some(Rgb {
        r: 204,
        g: 204,
        b: 204,
        a: 0.8,
    });
    assert_eq!(expected, get_rgb("#CCCC"));
    assert_eq!(expected, get_rgb("#CCCCCCCC"));
    assert_eq!(expected, get_rgb("rgb(204, 204, 204, 0.8)"));
    assert_eq!(expected, get_rgb("rgb(204 204 204 / 0.8)"));
    assert_eq!(expected, get_rgb("rgb(80%, 80%, 80%, 80%)"));
    assert_eq!(expected, get_rgb("rgb(80% 80% 80% / 80%)"));
}

#[wasm_bindgen_test]
fn rgb_alpha_zero() {
    let expected = Some(Rgb {
        r: 0,
        g: 0,
        b: 0,
        a: 0.0,
    });
    assert_eq!(expected, get_rgb("#0000"));
    assert_eq!(expected, get_rgb("#00000000"));
    assert_eq!(expected, get_rgb("rgb(0, 0, 0, 0)"));
    assert_eq!(expected, get_rgb("rgb(0 0 0 / 0)"));
    assert_eq!(expected, get_rgb("rgb(0%, 0%, 0%, 0%)"));
    assert_eq!(expected, get_rgb("rgb(0% 0% 0% / 0%)"));
}

#[wasm_bindgen_test]
fn rgb_alpha_range() {
    let expected = Some(Rgb {
        r: 255,
        g: 255,
        b: 255,
        a: 1.0,
    });
    assert_eq!(None, get_rgb("#GGG"));
    assert_eq!(None, get_rgb("#GGGGGG"));
    assert_eq!(None, get_rgb("rgb(510, 510, 510, 510)"));
    assert_eq!(None, get_rgb("rgb(510 510 510 / 510)"));
    assert_eq!(expected, get_rgb("rgb(200%, 200%, 200%, 200%)"));
    assert_eq!(expected, get_rgb("rgb(200% 200% 200% / 200%)"));
}

#[wasm_bindgen_test]
fn rgb_alpha_pos() {
    let expected = Some(Rgb {
        r: 204,
        g: 204,
        b: 204,
        a: 0.8,
    });
    assert_eq!(expected, get_rgb("rgb(+204, +204, +204, +0.8)"));
    assert_eq!(expected, get_rgb("rgb(+204 +204 +204 / +0.8)"));
    assert_eq!(expected, get_rgb("rgb(+80%, +80%, +80%, +80%)"));
    assert_eq!(expected, get_rgb("rgb(+80% +80% +80% / +80%)"));
}

#[wasm_bindgen_test]
fn rgb_alpha_neg() {
    let expected = Some(Rgb {
        r: 0,
        g: 0,
        b: 0,
        a: 0.0,
    });
    assert_eq!(None, get_rgb("rgb(-204, -204, -204, -0.8)"));
    assert_eq!(None, get_rgb("rgb(-204 -204 -204 / -0.8)"));
    assert_eq!(expected, get_rgb("rgb(-80%, -80%, -80%, -80%)"));
    assert_eq!(expected, get_rgb("rgb(-80% -80% -80% / -80%)"));
}

#[wasm_bindgen_test]
fn rgb_combined() {
    let expected = Some(Rgb {
        r: 204,
        g: 204,
        b: 204,
        a: 0.8,
    });

    let expected_2 = Some(Rgb {
        r: 0,
        g: 0,
        b: 0,
        a: 0.0,
    });

    assert_eq!(expected, get_rgb("rgb(+204, +204, 204, +0.8)"));
    assert_eq!(expected, get_rgb("rgb(+204 +204 204 / +0.8)"));
    assert_eq!(None, get_rgb("rgb(-204, -204, 204, -0.8)"));
    assert_eq!(None, get_rgb("rgb(-204 -204 204 / -0.8)"));

    assert_eq!(expected, get_rgb("rgb(+80%, +80%, 80%, +80%)"));
    assert_eq!(expected, get_rgb("rgb(+80% +80% 80% / +80%)"));
    assert_eq!(expected_2, get_rgb("rgb(-80%, -80%, 0%, -80%)"));
    assert_eq!(expected_2, get_rgb("rgb(-80% -80% 0% / -80%)"));
}

#[wasm_bindgen_test]
fn rgb_extra_at_start() {
    assert_eq!(None, get_rgb("1234rgb(50%, 50%, 50%)"));
    assert_eq!(None, get_rgb("1234rgb(50% 50% 50%)"));
}

#[wasm_bindgen_test]
fn rgb_extra_in_middle() {
    assert_eq!(None, get_rgb("rgb1(50%, 50%, 50%, 50%)"));
    assert_eq!(None, get_rgb("rgb1(50%, 1, 50%, 50%, 50%)"));
    assert_eq!(None, get_rgb("rgb(50%1, 50%, 50%, 50%)"));
    assert_eq!(None, get_rgb("rgb(50%, 50%1, 50% 50%1)"));
    assert_eq!(None, get_rgb("rgb(50%, 50%, 50%1, 50%)"));
    assert_eq!(None, get_rgb("rgb(50%, 50%, 50%, 50%1)"));

    assert_eq!(None, get_rgb("rgb1(50% 50% 50% 50%)"));
    assert_eq!(None, get_rgb("rgb(50% 1 50% 50% 50%)"));
    assert_eq!(None, get_rgb("rgb(50%1 50% 50% 50%)"));
    assert_eq!(None, get_rgb("rgb(50% 50%1 50% 50%)"));
    assert_eq!(None, get_rgb("rgb(50% 50% 50%1 50%)"));
    assert_eq!(None, get_rgb("rgb(50% 50% 50% 50%1)"));
}

#[wasm_bindgen_test]
fn rgb_extra_at_end() {
    assert_eq!(None, get_rgb("rgb(50%, 50%, 50%)1234"));
    assert_eq!(None, get_rgb("rgb(50% 50% 50%)1234"));
}

#[wasm_bindgen_test]
fn hsl_basic() {
    let expected = Some(Hsl {
        h: 240.0,
        s: 100.0,
        l: 50.5,
        a: 1.0,
    });
    assert_eq!(expected, get_hsl("hsl(240, 100%, 50.5%)"));
    assert_eq!(expected, get_hsl("hsl(240 100% 50.5%)"));
    assert_eq!(expected, get_hsl("hsla(240deg, 100%, 50.5%)"));
    assert_eq!(expected, get_hsl("hsla(240deg 100% 50.5%)"));
}

#[wasm_bindgen_test]
fn hsl_zero() {
    let expected = Some(Hsl {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 1.0,
    });
    assert_eq!(expected, get_hsl("hsl(0, 0%, 0%)"));
    assert_eq!(expected, get_hsl("hsl(0 0% 0%)"));
    assert_eq!(expected, get_hsl("hsla(0deg, 0%, 0%)"));
    assert_eq!(expected, get_hsl("hsla(0deg 0% 0%)"));
}

#[wasm_bindgen_test]
fn hsl_range() {
    let expected = Some(Hsl {
        h: 40.0,
        s: 10.0,
        l: 100.0,
        a: 1.0,
    });
    assert_eq!(expected, get_hsl("hsl(400, 10%, 200%)"));
    assert_eq!(expected, get_hsl("hsl(400 10% 200%)"));
    assert_eq!(expected, get_hsl("hsla(400deg, 10%, 200%)"));
    assert_eq!(expected, get_hsl("hsla(400deg 10% 200%)"));
}

#[wasm_bindgen_test]
fn hsl_pos() {
    let expected = Some(Hsl {
        h: 240.0,
        s: 100.0,
        l: 50.5,
        a: 1.0,
    });
    assert_eq!(expected, get_hsl("hsl(+240, +100%, +50.5%)"));
    assert_eq!(expected, get_hsl("hsl(+240 +100% +50.5%)"));
    assert_eq!(expected, get_hsl("hsla(+240deg, +100%, +50.5%)"));
    assert_eq!(expected, get_hsl("hsla(+240deg +100% +50.5%)"));
}

#[wasm_bindgen_test]
fn hsl_neg() {
    let expected = Some(Hsl {
        h: 120.0,
        s: 0.0,
        l: 0.0,
        a: 1.0,
    });
    assert_eq!(expected, get_hsl("hsl(-240, -100%, -50.5%)"));
    assert_eq!(expected, get_hsl("hsl(-240 -100% -50.5%)"));
    assert_eq!(expected, get_hsl("hsla(-240deg, -100%, -50.5%)"));
    assert_eq!(expected, get_hsl("hsla(-240deg -100% -50.5%)"));
}

#[wasm_bindgen_test]
fn hsl_alpha() {
    let expected = Some(Hsl {
        h: 240.0,
        s: 100.0,
        l: 50.5,
        a: 0.2,
    });
    assert_eq!(expected, get_hsl("hsl(240, 100%, 50.5%, 0.2)"));
    assert_eq!(expected, get_hsl("hsl(240 100% 50.5% / 0.2)"));
    assert_eq!(expected, get_hsl("hsla(240deg, 100%, 50.5%, 0.2)"));
    assert_eq!(expected, get_hsl("hsla(240deg 100% 50.5% / 0.2)"));

    assert_eq!(expected, get_hsl("hsl(240, 100%, 50.5%, 20%)"));
    assert_eq!(expected, get_hsl("hsl(240 100% 50.5% / 20%)"));
    assert_eq!(expected, get_hsl("hsla(240deg, 100%, 50.5%, 20%)"));
    assert_eq!(expected, get_hsl("hsla(240deg 100% 50.5% / 20%)"));
}

#[wasm_bindgen_test]
fn hsl_alpha_zero() {
    let expected = Some(Hsl {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 0.0,
    });
    assert_eq!(expected, get_hsl("hsl(0, 0%, 0%, 0)"));
    assert_eq!(expected, get_hsl("hsl(0 0% 0% / 0)"));
    assert_eq!(expected, get_hsl("hsla(0deg, 0%, 0%, 0)"));
    assert_eq!(expected, get_hsl("hsla(0deg 0% 0% / 0)"));

    assert_eq!(expected, get_hsl("hsl(0, 0%, 0%, 0%)"));
    assert_eq!(expected, get_hsl("hsl(0 0% 0% / 0%)"));
    assert_eq!(expected, get_hsl("hsla(0deg, 0%, 0%, 0%)"));
    assert_eq!(expected, get_hsl("hsla(0deg 0% 0% / 0%)"));
}

#[wasm_bindgen_test]
fn hsl_alpha_range() {
    let expected = Some(Hsl {
        h: 40.0,
        s: 10.0,
        l: 100.0,
        a: 1.0,
    });
    assert_eq!(expected, get_hsl("hsl(400, 10%, 200%, 10)"));
    assert_eq!(expected, get_hsl("hsl(400 10% 200% / 10)"));
    assert_eq!(expected, get_hsl("hsla(400deg, 10%, 200%, 10)"));
    assert_eq!(expected, get_hsl("hsla(400deg 10% 200% / 10)"));

    assert_eq!(expected, get_hsl("hsl(400, 10%, 200%, 1000%)"));
    assert_eq!(expected, get_hsl("hsl(400 10% 200% / 1000%)"));
    assert_eq!(expected, get_hsl("hsla(400deg, 10%, 200%, 1000%)"));
    assert_eq!(expected, get_hsl("hsla(400deg 10% 200% / 1000%)"));
}

#[wasm_bindgen_test]
fn hsl_alpha_pos() {
    let expected = Some(Hsl {
        h: 240.0,
        s: 100.0,
        l: 50.5,
        a: 0.8,
    });
    assert_eq!(expected, get_hsl("hsl(+240, +100%, +50.5%, +0.8)"));
    assert_eq!(expected, get_hsl("hsl(+240 +100% +50.5% / +0.8)"));
    assert_eq!(expected, get_hsl("hsla(+240deg, +100%, +50.5%, +0.8)"));
    assert_eq!(expected, get_hsl("hsla(+240deg +100% +50.5% / +0.8)"));

    assert_eq!(expected, get_hsl("hsl(+240, +100%, +50.5%, +80%)"));
    assert_eq!(expected, get_hsl("hsl(+240 +100% +50.5% / +80%)"));
    assert_eq!(expected, get_hsl("hsla(+240deg, +100%, +50.5%, +80%)"));
    assert_eq!(expected, get_hsl("hsla(+240deg +100% +50.5% / +80%)"));
}

#[wasm_bindgen_test]
fn hsl_alpha_neg() {
    let expected = Some(Hsl {
        h: 120.0,
        s: 0.0,
        l: 0.0,
        a: 0.0,
    });
    assert_eq!(expected, get_hsl("hsl(-240, -100%, -50.5%, -1)"));
    assert_eq!(expected, get_hsl("hsl(-240 -100% -50.5% / -1)"));
    assert_eq!(expected, get_hsl("hsla(-240deg, -100%, -50.5%, -1)"));
    assert_eq!(expected, get_hsl("hsla(-240deg -100% -50.5% / -1)"));

    assert_eq!(expected, get_hsl("hsl(-240, -100%, -50.5%, -100%)"));
    assert_eq!(expected, get_hsl("hsl(-240 -100% -50.5% / -100%)"));
    assert_eq!(expected, get_hsl("hsla(-240deg, -100%, -50.5%, -100%)"));
    assert_eq!(expected, get_hsl("hsla(-240deg -100% -50.5% / -100%)"));
}

#[wasm_bindgen_test]
fn hsl_combined() {
    let expected = Some(Hsl {
        h: 240.0,
        s: 100.0,
        l: 50.5,
        a: 0.202,
    });
    assert_eq!(expected, get_hsl("hsl(+240, 100%, 50.5%, 0.202)"));
    assert_eq!(expected, get_hsl("hsl(+240 100% 50.5% / 0.202)"));
    assert_eq!(expected, get_hsl("hsla(+240deg, 100%, 50.5%, 0.202)"));
    assert_eq!(expected, get_hsl("hsla(+240deg 100% 50.5% / 0.202)"));

    assert_eq!(expected, get_hsl("hsl(-120, 100%, 50.5%, 0.202)"));
    assert_eq!(expected, get_hsl("hsl(-120 100% 50.5% / 0.202)"));
    assert_eq!(expected, get_hsl("hsla(-120deg, 100%, 50.5%, 0.202)"));
    assert_eq!(expected, get_hsl("hsla(-120deg 100% 50.5% / 0.202)"));

    assert_eq!(expected, get_hsl("hsl(+240, 100%, 50.5%, 20.2%)"));
    assert_eq!(expected, get_hsl("hsl(+240 100% 50.5% / 20.2%)"));
    assert_eq!(expected, get_hsl("hsla(+240deg, 100%, 50.5%, 20.2%)"));
    assert_eq!(expected, get_hsl("hsla(+240deg 100% 50.5% / 20.2%)"));

    assert_eq!(expected, get_hsl("hsl(-120, 100%, 50.5%, 20.2%)"));
    assert_eq!(expected, get_hsl("hsl(-120 100% 50.5% / 20.2%)"));
    assert_eq!(expected, get_hsl("hsla(-120deg, 100%, 50.5%, 20.2%)"));
    assert_eq!(expected, get_hsl("hsla(-120deg 100% 50.5% / 20.2%)"));
}

#[wasm_bindgen_test]
fn hsl_extra_at_start() {
    assert_eq!(None, get_hsl("1234hsl(41, 50%, 45%)"));
    assert_eq!(None, get_hsl("1234hsl(41 50% 45%)"));
}

#[wasm_bindgen_test]
fn hsl_extra_in_middle() {
    assert_eq!(None, get_hsl("hsl1(50%, 50%, 50%)"));
    assert_eq!(None, get_hsl("hsl(50%, 1, 50%, 50%)"));
    assert_eq!(None, get_hsl("hsl(50%1, 50%, 50%)"));
    assert_eq!(None, get_hsl("hsl(50%, 50%1, 50%)"));
    assert_eq!(None, get_hsl("hsl(50%, 50%, 50%1)"));

    assert_eq!(None, get_hsl("hsl1(50% 50% 50%)"));
    assert_eq!(None, get_hsl("hsl1(50% 1 50% 50%)"));
    assert_eq!(None, get_hsl("hsl(50%1 50% 50%)"));
    assert_eq!(None, get_hsl("hsl(50% 50%1 50%)"));
    assert_eq!(None, get_hsl("hsl(50% 50% 50%1)"));
}

#[wasm_bindgen_test]
fn hsl_extra_at_end() {
    assert_eq!(None, get_hsl("hsl(41, 50%, 45%)1234"));
    assert_eq!(None, get_hsl("hsl(41 50% 45%)1234"));
}

#[wasm_bindgen_test]
fn hwb_basic() {
    let expected = Some(Hwb {
        h: 240.0,
        w: 100.0,
        b: 50.5,
        a: 1.0,
    });
    assert_eq!(expected, get_hwb("hwb(240, 100%, 50.5%)"));
    assert_eq!(expected, get_hwb("hwb(240 100% 50.5%)"));
    assert_eq!(expected, get_hwb("hwb(240deg, 100%, 50.5%)"));
    assert_eq!(expected, get_hwb("hwb(240deg 100% 50.5%)"));
}

#[wasm_bindgen_test]
fn hwb_zero() {
    let expected = Some(Hwb {
        h: 0.0,
        w: 0.0,
        b: 0.0,
        a: 1.0,
    });
    assert_eq!(expected, get_hwb("hwb(0, 0%, 0%)"));
    assert_eq!(expected, get_hwb("hwb(0 0% 0%)"));
    assert_eq!(expected, get_hwb("hwb(0deg, 0%, 0%)"));
    assert_eq!(expected, get_hwb("hwb(0deg 0% 0%)"));
}

#[wasm_bindgen_test]
fn hwb_range() {
    let expected = Some(Hwb {
        h: 40.0,
        w: 10.0,
        b: 100.0,
        a: 1.0,
    });
    assert_eq!(expected, get_hwb("hwb(400, 10%, 200%)"));
    assert_eq!(expected, get_hwb("hwb(400 10% 200%)"));
    assert_eq!(expected, get_hwb("hwb(400deg, 10%, 200%)"));
    assert_eq!(expected, get_hwb("hwb(400deg 10% 200%)"));
}

#[wasm_bindgen_test]
fn hwb_pos() {
    let expected = Some(Hwb {
        h: 240.0,
        w: 100.0,
        b: 50.5,
        a: 1.0,
    });
    assert_eq!(expected, get_hwb("hwb(+240, +100%, +50.5%)"));
    assert_eq!(expected, get_hwb("hwb(+240 +100% +50.5%)"));
    assert_eq!(expected, get_hwb("hwb(+240deg, +100%, +50.5%)"));
    assert_eq!(expected, get_hwb("hwb(+240deg +100% +50.5%)"));
}

#[wasm_bindgen_test]
fn hwb_neg() {
    let expected = Some(Hwb {
        h: 120.0,
        w: 0.0,
        b: 0.0,
        a: 1.0,
    });
    assert_eq!(expected, get_hwb("hwb(-240, -100%, -50.5%)"));
    assert_eq!(expected, get_hwb("hwb(-240 -100% -50.5%)"));
    assert_eq!(expected, get_hwb("hwb(-240deg, -100%, -50.5%)"));
    assert_eq!(expected, get_hwb("hwb(-240deg -100% -50.5%)"));
}

#[wasm_bindgen_test]
fn hwb_alpha() {
    let expected = Some(Hwb {
        h: 240.0,
        w: 100.0,
        b: 50.5,
        a: 0.2,
    });
    assert_eq!(expected, get_hwb("hwb(240, 100%, 50.5%, 0.2)"));
    assert_eq!(expected, get_hwb("hwb(240 100% 50.5% / 0.2)"));
    assert_eq!(expected, get_hwb("hwb(240deg, 100%, 50.5%, 0.2)"));
    assert_eq!(expected, get_hwb("hwb(240deg 100% 50.5% / 0.2)"));
}

#[wasm_bindgen_test]
fn hwb_alpha_zero() {
    let expected = Some(Hwb {
        h: 0.0,
        w: 0.0,
        b: 0.0,
        a: 0.0,
    });
    assert_eq!(expected, get_hwb("hwb(0, 0%, 0%, 0)"));
    assert_eq!(expected, get_hwb("hwb(0 0% 0% / 0)"));
    assert_eq!(expected, get_hwb("hwb(0deg, 0%, 0%, 0)"));
    assert_eq!(expected, get_hwb("hwb(0deg 0% 0% / 0)"));

    assert_eq!(expected, get_hwb("hwb(0, 0%, 0%, 0%)"));
    assert_eq!(expected, get_hwb("hwb(0 0% 0% / 0%)"));
    assert_eq!(expected, get_hwb("hwb(0deg, 0%, 0%, 0%)"));
    assert_eq!(expected, get_hwb("hwb(0deg 0% 0% / 0%)"));
}

#[wasm_bindgen_test]
fn hwb_alpha_range() {
    let expected = Some(Hwb {
        h: 40.0,
        w: 10.0,
        b: 100.0,
        a: 1.0,
    });
    assert_eq!(expected, get_hwb("hwb(400, 10%, 200%, 10)"));
    assert_eq!(expected, get_hwb("hwb(400 10% 200% / 10)"));
    assert_eq!(expected, get_hwb("hwb(400deg, 10%, 200%, 10)"));
    assert_eq!(expected, get_hwb("hwb(400deg 10% 200% / 10)"));

    assert_eq!(expected, get_hwb("hwb(400, 10%, 200%, 1000%)"));
    assert_eq!(expected, get_hwb("hwb(400 10% 200% / 1000%)"));
    assert_eq!(expected, get_hwb("hwb(400deg, 10%, 200%, 1000%)"));
    assert_eq!(expected, get_hwb("hwb(400deg 10% 200% / 1000%)"));
}

#[wasm_bindgen_test]
fn hwb_alpha_pos() {
    let expected = Some(Hwb {
        h: 240.0,
        w: 100.0,
        b: 50.5,
        a: 0.8,
    });
    assert_eq!(expected, get_hwb("hwb(+240, +100%, +50.5%, +0.8)"));
    assert_eq!(expected, get_hwb("hwb(+240 +100% +50.5% / +0.8)"));
    assert_eq!(expected, get_hwb("hwb(+240deg, +100%, +50.5%, +0.8)"));
    assert_eq!(expected, get_hwb("hwb(+240deg +100% +50.5% / +0.8)"));

    assert_eq!(expected, get_hwb("hwb(+240, +100%, +50.5%, +80%)"));
    assert_eq!(expected, get_hwb("hwb(+240 +100% +50.5% / +80%)"));
    assert_eq!(expected, get_hwb("hwb(+240deg, +100%, +50.5%, +80%)"));
    assert_eq!(expected, get_hwb("hwb(+240deg +100% +50.5% / +80%)"));
}

#[wasm_bindgen_test]
fn hwb_alpha_neg() {
    let expected = Some(Hwb {
        h: 120.0,
        w: 0.0,
        b: 0.0,
        a: 0.0,
    });
    assert_eq!(expected, get_hwb("hwb(-240, -100%, -50.5%, -1)"));
    assert_eq!(expected, get_hwb("hwb(-240 -100% -50.5% / -1)"));
    assert_eq!(expected, get_hwb("hwb(-240deg, -100%, -50.5%, -1)"));
    assert_eq!(expected, get_hwb("hwb(-240deg -100% -50.5% / -1)"));

    assert_eq!(expected, get_hwb("hwb(-240, -100%, -50.5%, -100%)"));
    assert_eq!(expected, get_hwb("hwb(-240 -100% -50.5% / -100%)"));
    assert_eq!(expected, get_hwb("hwb(-240deg, -100%, -50.5%, -100%)"));
    assert_eq!(expected, get_hwb("hwb(-240deg -100% -50.5% / -100%)"));
}

#[wasm_bindgen_test]
fn hwb_combined() {
    let expected = Some(Hwb {
        h: 240.0,
        w: 100.0,
        b: 50.5,
        a: 0.202,
    });
    assert_eq!(expected, get_hwb("hwb(+240, 100%, 50.5%, 0.202)"));
    assert_eq!(expected, get_hwb("hwb(+240 100% 50.5% / 0.202)"));
    assert_eq!(expected, get_hwb("hwb(+240deg, 100%, 50.5%, 0.202)"));
    assert_eq!(expected, get_hwb("hwb(+240deg 100% 50.5% / 0.202)"));

    assert_eq!(expected, get_hwb("hwb(-120, 100%, 50.5%, 0.202)"));
    assert_eq!(expected, get_hwb("hwb(-120 100% 50.5% / 0.202)"));
    assert_eq!(expected, get_hwb("hwb(-120deg, 100%, 50.5%, 0.202)"));
    assert_eq!(expected, get_hwb("hwb(-120deg 100% 50.5% / 0.202)"));

    assert_eq!(expected, get_hwb("hwb(+240, 100%, 50.5%, 20.2%)"));
    assert_eq!(expected, get_hwb("hwb(+240 100% 50.5% / 20.2%)"));
    assert_eq!(expected, get_hwb("hwb(+240deg, 100%, 50.5%, 20.2%)"));
    assert_eq!(expected, get_hwb("hwb(+240deg 100% 50.5% / 20.2%)"));

    assert_eq!(expected, get_hwb("hwb(-120, 100%, 50.5%, 20.2%)"));
    assert_eq!(expected, get_hwb("hwb(-120 100% 50.5% / 20.2%)"));
    assert_eq!(expected, get_hwb("hwb(-120deg, 100%, 50.5%, 20.2%)"));
    assert_eq!(expected, get_hwb("hwb(-120deg 100% 50.5% / 20.2%)"));
}

#[wasm_bindgen_test]
fn hwb_extra_at_start() {
    assert_eq!(None, get_hwb("1hwb(41, 50%, 45%)"));
    assert_eq!(None, get_hwb("1hwb(41 50% 45%)"));
}

#[wasm_bindgen_test]
fn hwb_extra_in_middle() {
    assert_eq!(None, get_hwb("hwb1(50%, 50%, 50%)"));
    assert_eq!(None, get_hwb("hwb(50%, 1, 50%, 50%)"));
    assert_eq!(None, get_hwb("hwb(50%1, 50%, 50%)"));
    assert_eq!(None, get_hwb("hwb(50%, 50%1, 50%)"));
    assert_eq!(None, get_hwb("hwb(50%, 50%, 50%1)"));

    assert_eq!(None, get_hwb("hwb1(50% 50% 50%)"));
    assert_eq!(None, get_hwb("hwb(50% 1 50% 50%)"));
    assert_eq!(None, get_hwb("hwb(50%1 50% 50%)"));
    assert_eq!(None, get_hwb("hwb(50% 50%1 50%)"));
    assert_eq!(None, get_hwb("hwb(50% 50% 50%1)"));
}

#[wasm_bindgen_test]
fn hwb_extra_at_end() {
    assert_eq!(None, get_hwb("hwb(41, 50%, 45%)1"));
    assert_eq!(None, get_hwb("hwb(41 50% 45%)1"));
}
