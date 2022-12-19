/*!
A simple badge generator in shields.io style, for Rust.

### Features
- Simple, bare-minimum API.
- Plastic, Flat, and Flat-Squared styles
- SVG output

### Example

```rust
use std::fs;
use std::io;
use ab_glyph::FontArc;
use shield_maker::{Renderer, Metadata, Style, FontFamily};

fn main() -> io::Result<()> {
    let font_bytes = fs::read("tests/resources/DejaVuSans.ttf")
        .expect("could not read DejaVuSans.ttf");
    let font = FontArc::try_from_vec(font_bytes)
        .expect("could not parse DejaVuSans.ttf");

    let meta = &Metadata {
        style: Style::Plastic,
        label: "coverage",
        message: "100%",
        font,
        font_family: FontFamily::Default,
        label_color: None,
        color: None,
    };

    let output = Renderer::render(meta);
    let raw_plastic_badge = fs::read("tests/resources/plastic_badge.svg")
        .expect("could not read plastic_badge.svg");
    let plastic_badge: String = String::from_utf8(raw_plastic_badge)
        .expect("failed converting plastic_badge.svg to String")
        .trim()
        .into();
    assert_eq!(plastic_badge, output);

    Ok(())
}
```
 */

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(missing_copy_implementations)]

mod color;
mod xml;
mod flat_square_style;
mod flat_style;
mod plastic_style;

mod badge;
pub use badge::{*};
