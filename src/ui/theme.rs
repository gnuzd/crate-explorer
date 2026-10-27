use ratatui::style::Color;

#[allow(dead_code)]
pub struct Theme {
    pub accent: Color,   // #E5683A
    pub border: Color,   // #3D322B
    pub text: Color,     // #E8DDD3
    pub muted: Color,    // #8A7B70
    pub bg_panel: Color, // #201A16
    pub bg: Color,       // #1A1512
    pub tan: Color,      // #DEA584
    pub warn: Color,     // #E6C15A
    pub danger: Color,   // #E06C5A
    pub ok: Color,       // #8BC77A
}

impl Theme {
    pub const fn crab() -> Self {
        Self {
            accent: Color::Rgb(0xE5, 0x68, 0x3A),
            border: Color::Rgb(0x3D, 0x32, 0x2B),
            text: Color::Rgb(0xE8, 0xDD, 0xD3),
            muted: Color::Rgb(0x8A, 0x7B, 0x70),
            bg_panel: Color::Rgb(0x20, 0x1A, 0x16),
            bg: Color::Rgb(0x1A, 0x15, 0x12),
            tan: Color::Rgb(0xDE, 0xA5, 0x84),
            warn: Color::Rgb(0xE6, 0xC1, 0x5A),
            danger: Color::Rgb(0xE0, 0x6C, 0x5A),
            ok: Color::Rgb(0x8B, 0xC7, 0x7A),
        }
    }
}
