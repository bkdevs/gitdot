/// taken from: https://github.com/boringdesigners/boring-avatars/blob/master/src/lib/components/avatar-beam.tsx
pub const SIZE: i32 = 36;
pub const COLORS: [&str; 5] = ["#92A1C6", "#146A7C", "#F0AB3D", "#C271B4", "#C20D90"];

pub fn hash_code(name: &str) -> i32 {
    let mut hash: i32 = 0;
    for ch in name.chars() {
        hash = hash
            .wrapping_shl(5)
            .wrapping_sub(hash)
            .wrapping_add(ch as i32);
    }
    hash.wrapping_abs()
}

pub fn get_digit(number: i32, ntn: u32) -> i32 {
    (number / 10_i32.pow(ntn)) % 10
}

pub fn get_unit(number: i32, range: i32, index: Option<u32>) -> i32 {
    let value = number % range;
    match index {
        Some(idx) if get_digit(number, idx) % 2 == 0 => -value,
        _ => value,
    }
}

pub fn get_boolean(number: i32, ntn: u32) -> bool {
    (number % 2_i32.pow(ntn)) != 0
}

pub fn get_random_color(number: i32) -> &'static str {
    COLORS[(number.wrapping_abs() as usize) % COLORS.len()]
}

pub fn get_contrast(hex: &str) -> &'static str {
    let h = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(0) as f64;
    let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(0) as f64;
    let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(0) as f64;
    if (r * 299.0 + g * 587.0 + b * 114.0) / 1000.0 >= 128.0 {
        "#000000"
    } else {
        "#ffffff"
    }
}

pub fn beam_svg(email: &str) -> String {
    let n = hash_code(email);

    let wrapper_color = get_random_color(n);
    let face_color = get_contrast(wrapper_color);
    let bg_color = get_random_color(n.wrapping_add(13));

    let pre_tx = get_unit(n, 10, Some(1));
    let wtx = if pre_tx < 5 {
        pre_tx + SIZE / 9
    } else {
        pre_tx
    };
    let pre_ty = get_unit(n, 10, Some(2));
    let wty = if pre_ty < 5 {
        pre_ty + SIZE / 9
    } else {
        pre_ty
    };

    let wr = get_unit(n, 360, None);
    let ws = 1.0 + get_unit(n, SIZE / 12, None) as f64 / 10.0;
    let wrx = if get_boolean(n, 1) { SIZE } else { SIZE / 6 };

    let mouth_spread = get_unit(n, 3, None);
    let eye_spread = get_unit(n, 5, None);
    let fr = get_unit(n, 10, Some(3));
    let ftx = if wtx > SIZE / 6 {
        wtx / 2
    } else {
        get_unit(n, 8, Some(1))
    };
    let fty = if wty > SIZE / 6 {
        wty / 2
    } else {
        get_unit(n, 7, Some(2))
    };

    let mouth = if get_boolean(n, 2) {
        format!(
            r#"<path d="M15 {}c2 1 4 1 6 0" stroke="{}" fill="none" stroke-linecap="round"/>"#,
            19 + mouth_spread,
            face_color,
        )
    } else {
        format!(
            r#"<path d="M13,{} a1,0.75 0 0,0 10,0" fill="{}"/>"#,
            19 + mouth_spread,
            face_color,
        )
    };

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 36 36" width="64" height="64">
<mask id="m"><rect width="36" height="36" rx="72" fill="white"/></mask>
<g mask="url(#m)">
<rect width="36" height="36" fill="{bg}"/>
<rect width="36" height="36" fill="{wc}" rx="{wrx}" transform="translate({wtx} {wty}) rotate({wr} 18 18) scale({ws:.1})"/>
<g transform="translate({ftx} {fty}) rotate({fr} 18 18)">
{mouth}
<rect x="{lex}" y="14" width="1.5" height="2" rx="1" fill="{fc}"/>
<rect x="{rex}" y="14" width="1.5" height="2" rx="1" fill="{fc}"/>
</g>
</g>
</svg>"#,
        bg = bg_color,
        wc = wrapper_color,
        wrx = wrx,
        wtx = wtx,
        wty = wty,
        wr = wr,
        ws = ws,
        ftx = ftx,
        fty = fty,
        fr = fr,
        mouth = mouth,
        fc = face_color,
        lex = 14 - eye_spread,
        rex = 20 + eye_spread,
    )
}
