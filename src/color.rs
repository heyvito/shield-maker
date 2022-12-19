#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(missing_copy_implementations)]

use css_color_parser::Color;

// WARNING: Those slices are searched through binary search. Keep its first
// components sorted.

const NAMED_COLORS: &[(&str, &str)] = &[
    ("blue", "#007ec6"),
    ("brightgreen", "#4c1"),
    ("green", "#97ca00"),
    ("grey", "#555"),
    ("lightgrey", "#9f9f9f"),
    ("orange", "#fe7d37"),
    ("red", "#e05d44"),
    ("yellow", "#dfb317"),
    ("yellowgreen", "#a4a61d"),
];

const ALIASES: &[(&str, &str)] = &[
    ("critical", "red"),
    ("gray", "grey"),
    ("important", "orange"),
    ("inactive", "lightgrey"),
    ("informational", "blue"),
    ("lightgray", "lightgrey"),
    ("success", "brightgreen"),
];

fn find_named_color(named: &str) -> Option<&str> {
    match NAMED_COLORS.binary_search_by(|(k, _)| (*k).cmp(named)) {
        Ok(idx) => Some(NAMED_COLORS[idx].1),
        Err(_) => None
    }
}

fn find_color_by_alias(named: &str) -> Option<&str> {
    match ALIASES.binary_search_by(|(k, _)| (*k).cmp(named)) {
        Ok(idx) => find_named_color(ALIASES[idx].1),
        Err(_) => None
    }
}

fn try_parse_color(color: &str) -> Option<Color> {
    match color.parse::<Color>() {
        Ok(c) => Some(c),
        Err(_) => None,
    }
}

pub(crate) fn color_by_name(name: Option<&str>) -> Option<Color> {
    let name = name?;
    let n = find_color_by_alias(name)
        .or_else(|| find_named_color(name))
        .unwrap_or(name);

    try_parse_color(n)
}

pub(crate) fn color_to_string(color: Color) -> String {
    format!("rgba({},{},{},{})", color.r, color.g, color.b, color.a)
}

pub(crate) fn brightness(color: Color) -> f32 {
    let c = color;
    f32::trunc(((c.r as f32 * 299.0 + c.g as f32 * 587.0 + c.b as f32 * 114.0) / 255000.0).abs() * 100.0) / 100.0
}

#[cfg(test)]
mod tests {
    use crate::color::*;

    #[test]
    fn finds_color_by_alias() {
        let color = find_color_by_alias("success");
        assert!(color.is_some());
        assert_eq!("#4c1", color.unwrap());
    }

    #[test]
    fn returns_none_when_alias_is_incorrect() {
        let color = find_color_by_alias("lol");
        assert!(color.is_none());
    }

    #[test]
    fn returns_none_when_named_color_is_unknown() {
        let color = find_named_color("lol");
        assert!(color.is_none());
    }

    #[test]
    fn returns_a_found_color() {
        let color = color_by_name(Some("success"));
        assert!(color.is_some());
        let color = color.unwrap();
        assert_eq!(color.a, 1.0);
        assert_eq!(color.r, 68_u8);
        assert_eq!(color.g, 204_u8);
        assert_eq!(color.b, 17_u8);
    }

    #[test]
    fn test_brightness() {
        assert_eq!(brightness(color_by_name(Some("white")).unwrap()), 1.0);
        assert_eq!(brightness(color_by_name(Some("black")).unwrap()), 0.0);
        assert_eq!(brightness(color_by_name(Some("#abc123")).unwrap()), 0.66);
        assert_eq!(brightness(color_by_name(Some("rgb(10, 5, 128)")).unwrap()), 0.08);
    }
}
