#[cfg(test)]
mod test {
    use std::fs;
    use std::path::PathBuf;
    use ab_glyph::FontArc;
    use shield_maker::{Renderer, Metadata, Style, FontFamily};

    fn get_resource(at: &str) -> Vec<u8> {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        d.push("resources");
        d.push(at);

        fs::read(&d)
            .unwrap_or_else(|err| panic!("Could not read {} at {}: {}", at, d.display(), err))
    }

    fn get_font() -> FontArc {
        let font_bytes = get_resource("DejaVuSans.ttf");
        FontArc::try_from_vec(font_bytes)
            .expect("could not parse font data into a font instance")
    }

    fn get_badge(name: &str) -> String {
        String::from_utf8(get_resource(name))
            .unwrap_or_else(|err| panic!("Error reading {} as UTF-8: {}", name, err))
            .trim()
            .into()
    }


    #[test]
    fn test_plastic_badge() {
        let f = get_font();

        let meta = &Metadata {
            style: Style::Plastic,
            label: "coverage",
            message: "100%",
            font: f,
            font_family: FontFamily::Default,
            label_color: None,
            color: None,
        };

        let output = Renderer::render(meta);
        assert_eq!(get_badge("plastic_badge.svg"), output);
    }

    #[test]
    fn test_flat_badge() {
        let f = get_font();

        let meta = &Metadata {
            style: Style::Flat,
            label: "coverage",
            message: "100%",
            font: f,
            font_family: FontFamily::Default,
            label_color: None,
            color: None,
        };

        let output = Renderer::render(meta);
        assert_eq!(get_badge("flat_badge.svg"), output);
    }

    #[test]
    fn test_flat_square_badge() {
        let f = get_font();

        let meta = &Metadata {
            style: Style::FlatSquare,
            label: "coverage",
            message: "100%",
            font: f,
            font_family: FontFamily::Default,
            label_color: None,
            color: None,
        };

        let output = Renderer::render(meta);
        assert_eq!(get_badge("flat_square_badge.svg"), output);
    }
}
