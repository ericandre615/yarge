use rusttype::{point, vector, Font, PositionedGlyph, Rect, Scale};

pub struct FontSize {
    pub scale: Scale
}

impl FontSize {
    pub fn new(size: f32) -> FontSize {
        FontSize { scale: Scale::uniform(size) }
    }
}

impl From<f32> for FontSize {
    fn from(size: f32) -> Self {
        FontSize::new(size)
    }
}

pub struct TextSettings {
    pub width: f32,
    pub font: String,
    pub size: FontSize,
    pub pos: (f32, f32),
    pub color: (u8, u8, u8, f32),
}

impl Default for TextSettings {
    fn default() -> Self {
        Self {
            width: 80.0,
            font: "".to_string(),
            size: 32.0.into(),
            pos: (0.0, 0.0),
            color: (255, 255, 255, 1.0),
        }
    }
}

pub struct Text {
    pub text: String,
    pub settings: TextSettings,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            settings: TextSettings { ..Default::default() },
        }
    }
}

impl Text {
    pub fn new(text: String, settings: TextSettings) -> Text {
        Text {
            text,
            settings
        }
    }
}
