# shield-maker
[![shield-maker](https://github.com/heyvito/shield-maker/actions/workflows/main.yml/badge.svg)](https://github.com/heyvito/shield-maker/actions/workflows/main.yml)
[![Crates.io](https://img.shields.io/crates/v/shield-maker.svg)](https://crates.io/crates/shield-maker)
[![Documentation](https://docs.rs/shield-maker/badge.svg)](https://docs.rs/shield-maker)
![](https://img.shields.io/badge/Rust-1.32+-orange.svg)
![](https://img.shields.io/badge/unsafe-forbidden-brightgreen.svg)

shield-maker implements a badge generator based on [Shields.io](https://github.com/badges/shields) implementation.

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
    let font_bytes = fs::read("verdana.ttf")
        .expect("could not read verdana.ttf");
    let font = FontArc::try_from_vec(font_bytes)
        .expect("could not parse verdana.ttf");

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
    println!("{}", output);
    // => <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width...

    Ok(())
}
```

### License

```
The MIT License (MIT)

Copyright (c) 2022-2023 Victor Gama de Oliveira

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

```
