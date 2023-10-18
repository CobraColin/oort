
#[derive(Clone)]
pub enum ColorName {
    Red,
    Green,
    Blue,
    Orange,
    Yellow,
    Purple,
    Pink,
    Brown,
    Cyan,
    Magenta,
    Teal,
    Lavender,
    Maroon,
    Olive,
    Coral,
    Indigo,
    Turquoise,
    DarkSlateGray,
    DarkMagenta,
    Gold,
    SeaGreen,
    Tomato,
    RoyalBlue,
    DarkOrange,
    Lime,
    DarkViolet,
    DarkGreen,
    DarkOliveGreen,
    DarkCyan,
    DarkGoldenrod,
    DarkSlateBlue,
    DarkRed,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSteelBlue,
    MediumAquamarine,
    MediumOrchid,
    MediumPurple,
    MediumSlateBlue,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    SlateBlue,
    SandyBrown,
    Silver,
    White,
}

pub struct ColorPalette {
    pub colors: [u32; 50],
}

impl ColorPalette {
    pub fn new() -> ColorPalette {
        let mut colors = [0; 50];

        colors[ColorName::Red as usize] = 0xFF0000;
        colors[ColorName::Green as usize] = 0x00FF00;
        colors[ColorName::Blue as usize] = 0x0000FF;
        colors[ColorName::Orange as usize] = 0xFFA500;
        colors[ColorName::Yellow as usize] = 0xFFFF00;
        colors[ColorName::Purple as usize] = 0x800080;
        colors[ColorName::Pink as usize] = 0xFFC0CB;
        colors[ColorName::Brown as usize] = 0xA52A2A;
        colors[ColorName::Cyan as usize] = 0x00FFFF;
        colors[ColorName::Magenta as usize] = 0xFF00FF;
        colors[ColorName::Teal as usize] = 0x008080;
        colors[ColorName::Lavender as usize] = 0xE6E6FA;
        colors[ColorName::Maroon as usize] = 0x800000;
        colors[ColorName::Olive as usize] = 0x808000;
        colors[ColorName::Coral as usize] = 0xFF6F61;
        colors[ColorName::Indigo as usize] = 0x4B0082;
        colors[ColorName::Turquoise as usize] = 0x40E0D0;
        colors[ColorName::DarkSlateGray as usize] = 0x2F4F4F;
        colors[ColorName::DarkMagenta as usize] = 0x8B008B;
        colors[ColorName::Gold as usize] = 0xFFD700;
        colors[ColorName::SeaGreen as usize] = 0x2E8B57;
        colors[ColorName::Tomato as usize] = 0xFF6347;
        colors[ColorName::RoyalBlue as usize] = 0x4169E1;
        colors[ColorName::DarkOrange as usize] = 0xFF8C00;
        colors[ColorName::Lime as usize] = 0x00FF00;
        colors[ColorName::DarkViolet as usize] = 0x9400D3;
        colors[ColorName::DarkGreen as usize] = 0x006400;
        colors[ColorName::DarkOliveGreen as usize] = 0x556B2F;
        colors[ColorName::DarkCyan as usize] = 0x008B8B;
        colors[ColorName::DarkGoldenrod as usize] = 0xB8860B;
        colors[ColorName::DarkSlateBlue as usize] = 0x483D8B;
        colors[ColorName::DarkRed as usize] = 0x8B0000;
        colors[ColorName::LightPink as usize] = 0xFFB6C1;
        colors[ColorName::LightSalmon as usize] = 0xFFA07A;
        colors[ColorName::LightSeaGreen as usize] = 0x20B2AA;
        colors[ColorName::LightSkyBlue as usize] = 0x87CEFA;
        colors[ColorName::LightSlateGray as usize] = 0x778899;
        colors[ColorName::LightSteelBlue as usize] = 0xB0C4DE;
        colors[ColorName::MediumAquamarine as usize] = 0x66CDAA;
        colors[ColorName::MediumOrchid as usize] = 0xBA55D3;
        colors[ColorName::MediumPurple as usize] = 0x9370DB;
        colors[ColorName::MediumSlateBlue as usize] = 0x7B68EE;
        colors[ColorName::MediumTurquoise as usize] = 0x48D1CC;
        colors[ColorName::MediumVioletRed as usize] = 0xC71585;
        colors[ColorName::MidnightBlue as usize] = 0x191970;
        colors[ColorName::SlateBlue as usize] = 0x6A5ACD;
        colors[ColorName::SandyBrown as usize] = 0xF4A460;
        colors[ColorName::Silver as usize] = 0xC0C0C0;
        colors[ColorName::White as usize] = 0xFFFFFF;

        ColorPalette { colors }
    }

    pub fn get_color(&self, color_name: ColorName) -> u32 {
        self.colors[color_name as usize]
    }
}