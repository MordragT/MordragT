use std::{fs, io};

use ansi2::{css::Mode, theme::Theme};

#[derive(Debug, Clone, Copy)]
pub struct Options {
    theme: Theme,
    width: Option<usize>,
    font: &'static str,
    light_bg: &'static str,
    dark_bg: &'static str,
    font_size: usize,
    length_adjust: Option<&'static str>,
    sourcemap: bool,
}

impl Options {
    pub const DEFAULT: Self = Self {
        theme: Theme::Vga,
        width: None,
        font: "Adwaita Mono",
        light_bg: "#FFFFFF",
        dark_bg: "#101414",
        font_size: 14,
        length_adjust: None,
        sourcemap: false,
    };
}

impl Default for Options {
    fn default() -> Self {
        Self::DEFAULT
    }
}

pub fn to_svg(contents: &str, options: Options) -> io::Result<()> {
    let Options {
        theme,
        width,
        font,
        light_bg,
        dark_bg,
        font_size,
        length_adjust,
        sourcemap,
    } = options;

    let convert = |mode| {
        ansi2::svg::to_svg(
            contents,
            theme,
            width,
            Some(font.to_owned()),
            Some(mode),
            Some(light_bg.to_owned()),
            Some(dark_bg.to_owned()),
            Some(font_size),
            length_adjust.map(ToOwned::to_owned),
            sourcemap,
        )
    };

    let dark = convert(Mode::Dark);
    let light = convert(Mode::Light);

    fs::write("dark.svg", dark)?;
    fs::write("light.svg", light)?;

    Ok(())
}
