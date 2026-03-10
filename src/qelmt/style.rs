use hex_color::HexColor;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Default)]
pub(crate) enum LineStyle {
    #[default]
    Normal,
    Dashed,
    Dotted,
    DotDash,
}

impl Display for LineStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Normal => "normal",
                Self::Dashed => "dashed",
                Self::Dotted => "dotted",
                Self::DotDash => "dashdotted",
            }
        )
    }
}

#[derive(Debug, Default)]
pub(crate) enum LineWeight {
    None,
    Thin,
    #[default]
    Normal,
    Strong,
    High,
}

impl Display for LineWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "none",
                Self::Thin => "thin",
                Self::Normal => "normal",

                //By process of elimination this must be "Strong"..not sure where that word comes fromt
                //it doesn't appear to be French for Strong...just need to very that's what this actually is
                // TODO: Verify this
                Self::Strong => "eleve",

                //This looks to me like a mis-translation, but it's what QET uses, so it needs to match
                Self::High => "hight",
            }
        )
    }
}
#[derive(Debug, Default)]
pub struct QETColor {
    color: Option<HexColor>,
}

impl From<HexColor> for QETColor {
    fn from(color: HexColor) -> Self {
        Self { color: Some(color) }
    }
}

#[allow(clippy::too_many_lines)]
impl Display for QETColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.color {
                None => "none",
                Some(color) => {
                    match color {
                        //https://doc.qt.io/archives/qt-5.15/qcolor.html
                        //https://doc.qt.io/qt-6/qcolor.html
                        //QET: elementpicturefactory.cpp
                        HexColor {
                            r: 217,
                            g: 0,
                            b: 0,
                            a: _,
                        } => "red",
                        HexColor {
                            r: 0,
                            g: 0,
                            b: 217,
                            a: _,
                        } => "blue",
                        HexColor {
                            r: 0,
                            g: 217,
                            b: 0,
                            a: _,
                        } => "green",
                        HexColor {
                            r: 136,
                            g: 136,
                            b: 141,
                            a: _,
                        } => "gray",
                        HexColor {
                            r: 97,
                            g: 44,
                            b: 0,
                            a: _,
                        } => "brun",
                        HexColor {
                            r: 217,
                            g: 217,
                            b: 0,
                            a: _,
                        } => "yellow",
                        HexColor {
                            r: 0,
                            g: 217,
                            b: 217,
                            a: _,
                        } => "cyan",
                        HexColor {
                            r: 217,
                            g: 0,
                            b: 217,
                            a: _,
                        } => "magenta",
                        HexColor {
                            r: 163,
                            g: 163,
                            b: 163,
                            a: _,
                        } => "lightgray",
                        HexColor {
                            r: 255,
                            g: 128,
                            b: 0,
                            a: _,
                        } => "orange",
                        HexColor {
                            r: 136,
                            g: 28,
                            b: 168,
                            a: _,
                        } => "purple",
                        HexColor {
                            r: 255,
                            g: 192,
                            b: 203,
                            a: _,
                        } => "HTMLPinkPink",
                        HexColor {
                            r: 255,
                            g: 182,
                            b: 193,
                            a: _,
                        } => "HTMLPinkLightPink",
                        HexColor {
                            r: 255,
                            g: 105,
                            b: 180,
                            a: _,
                        } => "HTMLPinkHotPink",
                        HexColor {
                            r: 255,
                            g: 20,
                            b: 147,
                            a: _,
                        } => "HTMLPinkDeepPink",
                        HexColor {
                            r: 219,
                            g: 112,
                            b: 147,
                            a: _,
                        } => "HTMLPinkPaleVioletRed",
                        HexColor {
                            r: 255,
                            g: 160,
                            b: 122,
                            a: _,
                        } => "HTMLRedLightSalmon",
                        HexColor {
                            r: 250,
                            g: 128,
                            b: 114,
                            a: _,
                        } => "HTMLRedSalmon",
                        HexColor {
                            r: 233,
                            g: 150,
                            b: 122,
                            a: _,
                        } => "HTMLRedDarkSalmon",
                        HexColor {
                            r: 240,
                            g: 128,
                            b: 128,
                            a: _,
                        } => "HTMLRedLightCoral",
                        HexColor {
                            r: 205,
                            g: 92,
                            b: 92,
                            a: _,
                        } => "HTMLRedIndianRed",
                        HexColor {
                            r: 220,
                            g: 20,
                            b: 60,
                            a: _,
                        } => "HTMLRedCrimson",
                        HexColor {
                            r: 178,
                            g: 34,
                            b: 34,
                            a: _,
                        } => "HTMLRedFirebrick",
                        HexColor {
                            r: 139,
                            g: 0,
                            b: 0,
                            a: _,
                        } => "HTMLRedDarkRed",
                        HexColor {
                            r: 255,
                            g: 0,
                            b: 0,
                            a: _,
                        } => "HTMLRedRed",
                        HexColor {
                            r: 255,
                            g: 69,
                            b: 0,
                            a: _,
                        } => "HTMLOrangeOrangeRed",
                        HexColor {
                            r: 255,
                            g: 99,
                            b: 71,
                            a: _,
                        } => "HTMLOrangeTomato",
                        HexColor {
                            r: 255,
                            g: 127,
                            b: 80,
                            a: _,
                        } => "HTMLOrangeCoral",
                        HexColor {
                            r: 255,
                            g: 140,
                            b: 0,
                            a: _,
                        } => "HTMLOrangeDarkOrange",
                        HexColor {
                            r: 255,
                            g: 165,
                            b: 0,
                            a: _,
                        } => "HTMLOrangeOrange",
                        HexColor {
                            r: 255,
                            g: 255,
                            b: 0,
                            a: _,
                        } => "HTMLYellowYellow",
                        HexColor {
                            r: 255,
                            g: 255,
                            b: 224,
                            a: _,
                        } => "HTMLYellowLightYellow",
                        HexColor {
                            r: 255,
                            g: 250,
                            b: 205,
                            a: _,
                        } => "HTMLYellowLemonChiffon",
                        HexColor {
                            r: 250,
                            g: 250,
                            b: 210,
                            a: _,
                        } => "HTMLYellowLightGoldenrodYellow",
                        HexColor {
                            r: 255,
                            g: 239,
                            b: 213,
                            a: _,
                        } => "HTMLYellowPapayaWhip",
                        HexColor {
                            r: 255,
                            g: 228,
                            b: 181,
                            a: _,
                        } => "HTMLYellowMoccasin",
                        HexColor {
                            r: 255,
                            g: 218,
                            b: 185,
                            a: _,
                        } => "HTMLYellowPeachPuff",
                        HexColor {
                            r: 238,
                            g: 232,
                            b: 170,
                            a: _,
                        } => "HTMLYellowPaleGoldenrod",
                        HexColor {
                            r: 240,
                            g: 230,
                            b: 140,
                            a: _,
                        } => "HTMLYellowKhaki",
                        HexColor {
                            r: 189,
                            g: 183,
                            b: 107,
                            a: _,
                        } => "HTMLYellowDarkKhaki",
                        HexColor {
                            r: 255,
                            g: 215,
                            b: 0,
                            a: _,
                        } => "HTMLYellowGold",
                        HexColor {
                            r: 255,
                            g: 248,
                            b: 220,
                            a: _,
                        } => "HTMLBrownCornsilk",
                        HexColor {
                            r: 255,
                            g: 235,
                            b: 205,
                            a: _,
                        } => "HTMLBrownBlanchedAlmond",
                        HexColor {
                            r: 255,
                            g: 228,
                            b: 196,
                            a: _,
                        } => "HTMLBrownBisque",
                        HexColor {
                            r: 255,
                            g: 222,
                            b: 173,
                            a: _,
                        } => "HTMLBrownNavajoWhite",
                        HexColor {
                            r: 245,
                            g: 222,
                            b: 179,
                            a: _,
                        } => "HTMLBrownWheat",
                        HexColor {
                            r: 222,
                            g: 184,
                            b: 135,
                            a: _,
                        } => "HTMLBrownBurlywood",
                        HexColor {
                            r: 210,
                            g: 180,
                            b: 140,
                            a: _,
                        } => "HTMLBrownTan",
                        HexColor {
                            r: 188,
                            g: 143,
                            b: 143,
                            a: _,
                        } => "HTMLBrownRosyBrown",
                        HexColor {
                            r: 244,
                            g: 164,
                            b: 96,
                            a: _,
                        } => "HTMLBrownSandyBrown",
                        HexColor {
                            r: 218,
                            g: 165,
                            b: 32,
                            a: _,
                        } => "HTMLBrownGoldenrod",
                        HexColor {
                            r: 184,
                            g: 134,
                            b: 11,
                            a: _,
                        } => "HTMLBrownDarkGoldenrod",
                        HexColor {
                            r: 205,
                            g: 133,
                            b: 63,
                            a: _,
                        } => "HTMLBrownPeru",
                        HexColor {
                            r: 210,
                            g: 105,
                            b: 30,
                            a: _,
                        } => "HTMLBrownChocolate",
                        HexColor {
                            r: 139,
                            g: 69,
                            b: 19,
                            a: _,
                        } => "HTMLBrownSaddleBrown",
                        HexColor {
                            r: 160,
                            g: 82,
                            b: 45,
                            a: _,
                        } => "HTMLBrownSienna",
                        HexColor {
                            r: 165,
                            g: 42,
                            b: 42,
                            a: _,
                        } => "HTMLBrownBrown",
                        HexColor {
                            r: 128,
                            g: 0,
                            b: 0,
                            a: _,
                        } => "HTMLBrownMaroon",
                        HexColor {
                            r: 85,
                            g: 107,
                            b: 47,
                            a: _,
                        } => "HTMLGreenDarkOliveGreen",
                        HexColor {
                            r: 128,
                            g: 128,
                            b: 0,
                            a: _,
                        } => "HTMLGreenOlive",
                        HexColor {
                            r: 107,
                            g: 142,
                            b: 35,
                            a: _,
                        } => "HTMLGreenOliveDrab",
                        HexColor {
                            r: 154,
                            g: 205,
                            b: 50,
                            a: _,
                        } => "HTMLGreenYellowGreen",
                        HexColor {
                            r: 50,
                            g: 205,
                            b: 50,
                            a: _,
                        } => "HTMLGreenLimeGreen",
                        HexColor {
                            r: 0,
                            g: 255,
                            b: 0,
                            a: _,
                        } => "HTMLGreenLime",
                        HexColor {
                            r: 124,
                            g: 252,
                            b: 0,
                            a: _,
                        } => "HTMLGreenLawnGreen",
                        HexColor {
                            r: 127,
                            g: 255,
                            b: 0,
                            a: _,
                        } => "HTMLGreenChartreuse",
                        HexColor {
                            r: 173,
                            g: 255,
                            b: 47,
                            a: _,
                        } => "HTMLGreenGreenYellow",
                        HexColor {
                            r: 0,
                            g: 255,
                            b: 127,
                            a: _,
                        } => "HTMLGreenSpringGreen",
                        HexColor {
                            r: 0,
                            g: 250,
                            b: 154,
                            a: _,
                        } => "HTMLGreenMediumSpringGreen",
                        HexColor {
                            r: 144,
                            g: 238,
                            b: 144,
                            a: _,
                        } => "HTMLGreenLightGreen",
                        HexColor {
                            r: 152,
                            g: 251,
                            b: 152,
                            a: _,
                        } => "HTMLGreenPaleGreen",
                        HexColor {
                            r: 143,
                            g: 188,
                            b: 143,
                            a: _,
                        } => "HTMLGreenDarkSeaGreen",
                        HexColor {
                            r: 102,
                            g: 205,
                            b: 170,
                            a: _,
                        } => "HTMLGreenMediumAquamarine",
                        HexColor {
                            r: 60,
                            g: 179,
                            b: 113,
                            a: _,
                        } => "HTMLGreenMediumSeaGreen",
                        HexColor {
                            r: 46,
                            g: 139,
                            b: 87,
                            a: _,
                        } => "HTMLGreenSeaGreen",
                        HexColor {
                            r: 34,
                            g: 139,
                            b: 34,
                            a: _,
                        } => "HTMLGreenForestGreen",
                        HexColor {
                            r: 0,
                            g: 128,
                            b: 0,
                            a: _,
                        } => "HTMLGreenGreen",
                        HexColor {
                            r: 0,
                            g: 100,
                            b: 0,
                            a: _,
                        } => "HTMLGreenDarkGreen",
                        HexColor {
                            r: 0,
                            g: 255,
                            b: 255,
                            a: _,
                        } => "HTMLCyanCyan",
                        HexColor {
                            r: 224,
                            g: 255,
                            b: 255,
                            a: _,
                        } => "HTMLCyanLightCyan",
                        HexColor {
                            r: 175,
                            g: 238,
                            b: 238,
                            a: _,
                        } => "HTMLCyanPaleTurquoise",
                        HexColor {
                            r: 127,
                            g: 255,
                            b: 212,
                            a: _,
                        } => "HTMLCyanAquamarine",
                        HexColor {
                            r: 64,
                            g: 224,
                            b: 208,
                            a: _,
                        } => "HTMLCyanTurquoise",
                        HexColor {
                            r: 72,
                            g: 209,
                            b: 204,
                            a: _,
                        } => "HTMLCyanMediumTurquoise",
                        HexColor {
                            r: 0,
                            g: 206,
                            b: 209,
                            a: _,
                        } => "HTMLCyanDarkTurquoise",
                        HexColor {
                            r: 32,
                            g: 178,
                            b: 170,
                            a: _,
                        } => "HTMLCyanLightSeaGreen",
                        HexColor {
                            r: 95,
                            g: 158,
                            b: 160,
                            a: _,
                        } => "HTMLCyanCadetBlue",
                        HexColor {
                            r: 0,
                            g: 139,
                            b: 139,
                            a: _,
                        } => "HTMLCyanDarkCyan",
                        HexColor {
                            r: 0,
                            g: 128,
                            b: 128,
                            a: _,
                        } => "HTMLCyanTeal",
                        HexColor {
                            r: 176,
                            g: 196,
                            b: 222,
                            a: _,
                        } => "HTMLBlueLightSteelBlue",
                        HexColor {
                            r: 176,
                            g: 224,
                            b: 230,
                            a: _,
                        } => "HTMLBluePowderBlue",
                        HexColor {
                            r: 173,
                            g: 216,
                            b: 230,
                            a: _,
                        } => "HTMLBlueLightBlue",
                        HexColor {
                            r: 135,
                            g: 206,
                            b: 235,
                            a: _,
                        } => "HTMLBlueSkyBlue",
                        HexColor {
                            r: 135,
                            g: 206,
                            b: 250,
                            a: _,
                        } => "HTMLBlueLightSkyBlue",
                        HexColor {
                            r: 0,
                            g: 191,
                            b: 255,
                            a: _,
                        } => "HTMLBlueDeepSkyBlue",
                        HexColor {
                            r: 30,
                            g: 144,
                            b: 255,
                            a: _,
                        } => "HTMLBlueDodgerBlue",
                        HexColor {
                            r: 100,
                            g: 149,
                            b: 237,
                            a: _,
                        } => "HTMLBlueCornflowerBlue",
                        HexColor {
                            r: 70,
                            g: 130,
                            b: 180,
                            a: _,
                        } => "HTMLBlueSteelBlue",
                        HexColor {
                            r: 65,
                            g: 105,
                            b: 225,
                            a: _,
                        } => "HTMLBlueRoyalBlue",
                        HexColor {
                            r: 0,
                            g: 0,
                            b: 255,
                            a: _,
                        } => "HTMLBlueBlue",
                        HexColor {
                            r: 0,
                            g: 0,
                            b: 205,
                            a: _,
                        } => "HTMLBlueMediumBlue",
                        HexColor {
                            r: 0,
                            g: 0,
                            b: 139,
                            a: _,
                        } => "HTMLBlueDarkBlue",
                        HexColor {
                            r: 0,
                            g: 0,
                            b: 128,
                            a: _,
                        } => "HTMLBlueNavy",
                        HexColor {
                            r: 25,
                            g: 25,
                            b: 112,
                            a: _,
                        } => "HTMLBlueMidnightBlue",
                        HexColor {
                            r: 230,
                            g: 230,
                            b: 250,
                            a: _,
                        } => "HTMLPurpleLavender",
                        HexColor {
                            r: 216,
                            g: 191,
                            b: 216,
                            a: _,
                        } => "HTMLPurpleThistle",
                        HexColor {
                            r: 221,
                            g: 160,
                            b: 221,
                            a: _,
                        } => "HTMLPurplePlum",
                        HexColor {
                            r: 238,
                            g: 130,
                            b: 238,
                            a: _,
                        } => "HTMLPurpleViolet",
                        HexColor {
                            r: 218,
                            g: 112,
                            b: 214,
                            a: _,
                        } => "HTMLPurpleOrchid",
                        HexColor {
                            r: 255,
                            g: 0,
                            b: 255,
                            a: _,
                        } => "HTMLPurpleFuchsia",
                        HexColor {
                            r: 186,
                            g: 85,
                            b: 211,
                            a: _,
                        } => "HTMLPurpleMediumOrchid",
                        HexColor {
                            r: 147,
                            g: 112,
                            b: 219,
                            a: _,
                        } => "HTMLPurpleMediumPurple",
                        HexColor {
                            r: 138,
                            g: 43,
                            b: 226,
                            a: _,
                        } => "HTMLPurpleBlueViolet",
                        HexColor {
                            r: 148,
                            g: 0,
                            b: 211,
                            a: _,
                        } => "HTMLPurpleDarkViolet",
                        HexColor {
                            r: 153,
                            g: 50,
                            b: 204,
                            a: _,
                        } => "HTMLPurpleDarkOrchid",
                        HexColor {
                            r: 139,
                            g: 0,
                            b: 139,
                            a: _,
                        } => "HTMLPurpleDarkMagenta",
                        HexColor {
                            r: 128,
                            g: 0,
                            b: 128,
                            a: _,
                        } => "HTMLPurplePurple",
                        HexColor {
                            r: 75,
                            g: 0,
                            b: 130,
                            a: _,
                        } => "HTMLPurpleIndigo",
                        HexColor {
                            r: 72,
                            g: 61,
                            b: 139,
                            a: _,
                        } => "HTMLPurpleDarkSlateBlue",
                        HexColor {
                            r: 106,
                            g: 90,
                            b: 205,
                            a: _,
                        } => "HTMLPurpleSlateBlue",
                        HexColor {
                            r: 123,
                            g: 104,
                            b: 238,
                            a: _,
                        } => "HTMLPurpleMediumSlateBlue",
                        HexColor {
                            r: 255,
                            g: 255,
                            b: 255,
                            a: _,
                        } => "HTMLWhiteWhite",
                        HexColor {
                            r: 255,
                            g: 250,
                            b: 250,
                            a: _,
                        } => "HTMLWhiteSnow",
                        HexColor {
                            r: 240,
                            g: 255,
                            b: 240,
                            a: _,
                        } => "HTMLWhiteHoneydew",
                        HexColor {
                            r: 245,
                            g: 255,
                            b: 250,
                            a: _,
                        } => "HTMLWhiteMintCream",
                        HexColor {
                            r: 240,
                            g: 255,
                            b: 255,
                            a: _,
                        } => "HTMLWhiteAzure",
                        HexColor {
                            r: 240,
                            g: 248,
                            b: 255,
                            a: _,
                        } => "HTMLWhiteAliceBlue",
                        HexColor {
                            r: 248,
                            g: 248,
                            b: 255,
                            a: _,
                        } => "HTMLWhiteGhostWhite",
                        HexColor {
                            r: 245,
                            g: 245,
                            b: 245,
                            a: _,
                        } => "HTMLWhiteWhiteSmoke",
                        HexColor {
                            r: 255,
                            g: 245,
                            b: 238,
                            a: _,
                        } => "HTMLWhiteSeashell",
                        HexColor {
                            r: 245,
                            g: 245,
                            b: 220,
                            a: _,
                        } => "HTMLWhiteBeige",
                        HexColor {
                            r: 253,
                            g: 245,
                            b: 230,
                            a: _,
                        } => "HTMLWhiteOldLace",
                        HexColor {
                            r: 255,
                            g: 250,
                            b: 240,
                            a: _,
                        } => "HTMLWhiteFloralWhite",
                        HexColor {
                            r: 255,
                            g: 255,
                            b: 240,
                            a: _,
                        } => "HTMLWhiteIvory",
                        HexColor {
                            r: 250,
                            g: 235,
                            b: 215,
                            a: _,
                        } => "HTMLWhiteAntiqueWhite",
                        HexColor {
                            r: 250,
                            g: 240,
                            b: 230,
                            a: _,
                        } => "HTMLWhiteLinen",
                        HexColor {
                            r: 255,
                            g: 240,
                            b: 245,
                            a: _,
                        } => "HTMLWhiteLavenderBlush",
                        HexColor {
                            r: 255,
                            g: 228,
                            b: 225,
                            a: _,
                        } => "HTMLWhiteMistyRose",
                        HexColor {
                            r: 220,
                            g: 220,
                            b: 220,
                            a: _,
                        } => "HTMLGrayGainsboro",
                        HexColor {
                            r: 211,
                            g: 211,
                            b: 211,
                            a: _,
                        } => "HTMLGrayLightGray",
                        HexColor {
                            r: 192,
                            g: 192,
                            b: 192,
                            a: _,
                        } => "HTMLGraySilver",
                        HexColor {
                            r: 169,
                            g: 169,
                            b: 169,
                            a: _,
                        } => "HTMLGrayDarkGray",
                        HexColor {
                            r: 128,
                            g: 128,
                            b: 128,
                            a: _,
                        } => "HTMLGrayGray",
                        HexColor {
                            r: 105,
                            g: 105,
                            b: 105,
                            a: _,
                        } => "HTMLGrayDimGray",
                        HexColor {
                            r: 119,
                            g: 136,
                            b: 153,
                            a: _,
                        } => "HTMLGrayLightSlateGray",
                        HexColor {
                            r: 112,
                            g: 128,
                            b: 144,
                            a: _,
                        } => "HTMLGraySlateGray",
                        HexColor {
                            r: 47,
                            g: 79,
                            b: 79,
                            a: _,
                        } => "HTMLGrayDarkSlateGray",
                        HexColor {
                            r: 0,
                            g: 0,
                            b: 0,
                            a: _,
                        } => "HTMLGrayBlack",

                        //what do do if the is not one of the set ones. Looks like QET
                        //doesn't support hex values in the style string...if you manually
                        //add a hex string it just defautls to black...I guess that's not the
                        //end of the world..could just output the hex color and let QET do what
                        //it's going to do...or should I try and somehow find the closest valid color?
                        //that would probably be more accurate, but trickier.
                        _ => todo!(),
                    }
                }
            }
        )
    }
}

#[allow(clippy::too_many_lines)]
impl FromStr for QETColor {
    type Err = &'static str; //add better error later;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self {
                color: Some(HexColor {
                    r: 217,
                    g: 0,
                    b: 0,
                    a: 255,
                }),
            }),
            "blue" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 0,
                    b: 217,
                    a: 255,
                }),
            }),
            "green" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 217,
                    b: 0,
                    a: 255,
                }),
            }),
            "gray" => Ok(Self {
                color: Some(HexColor {
                    r: 136,
                    g: 136,
                    b: 141,
                    a: 255,
                }),
            }),
            "brun" => Ok(Self {
                color: Some(HexColor {
                    r: 97,
                    g: 44,
                    b: 0,
                    a: 255,
                }),
            }),
            "yellow" => Ok(Self {
                color: Some(HexColor {
                    r: 217,
                    g: 217,
                    b: 0,
                    a: 255,
                }),
            }),
            "cyan" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 217,
                    b: 217,
                    a: 255,
                }),
            }),
            "magenta" => Ok(Self {
                color: Some(HexColor {
                    r: 217,
                    g: 0,
                    b: 217,
                    a: 255,
                }),
            }),
            "lightgray" => Ok(Self {
                color: Some(HexColor {
                    r: 163,
                    g: 163,
                    b: 163,
                    a: 255,
                }),
            }),
            "orange" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 128,
                    b: 0,
                    a: 255,
                }),
            }),
            "purple" => Ok(Self {
                color: Some(HexColor {
                    r: 136,
                    g: 28,
                    b: 168,
                    a: 255,
                }),
            }),
            "HTMLPinkPink" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 192,
                    b: 203,
                    a: 255,
                }),
            }),
            "HTMLPinkLightPink" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 182,
                    b: 193,
                    a: 255,
                }),
            }),
            "HTMLPinkHotPink" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 105,
                    b: 180,
                    a: 255,
                }),
            }),
            "HTMLPinkDeepPink" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 20,
                    b: 147,
                    a: 255,
                }),
            }),
            "HTMLPinkPaleVioletRed" => Ok(Self {
                color: Some(HexColor {
                    r: 219,
                    g: 112,
                    b: 147,
                    a: 255,
                }),
            }),
            "HTMLPinkMediumVioletRed" => Ok(Self {
                color: Some(HexColor {
                    r: 199,
                    g: 21,
                    b: 133,
                    a: 255,
                }),
            }),
            "HTMLRedLightSalmon" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 160,
                    b: 122,
                    a: 255,
                }),
            }),
            "HTMLRedSalmon" => Ok(Self {
                color: Some(HexColor {
                    r: 250,
                    g: 128,
                    b: 114,
                    a: 255,
                }),
            }),
            "HTMLRedDarkSalmon" => Ok(Self {
                color: Some(HexColor {
                    r: 233,
                    g: 150,
                    b: 122,
                    a: 255,
                }),
            }),
            "HTMLRedLightCoral" => Ok(Self {
                color: Some(HexColor {
                    r: 240,
                    g: 128,
                    b: 128,
                    a: 255,
                }),
            }),
            "HTMLRedIndianRed" => Ok(Self {
                color: Some(HexColor {
                    r: 205,
                    g: 92,
                    b: 92,
                    a: 255,
                }),
            }),
            "HTMLRedCrimson" => Ok(Self {
                color: Some(HexColor {
                    r: 220,
                    g: 20,
                    b: 60,
                    a: 255,
                }),
            }),
            "HTMLRedFirebrick" => Ok(Self {
                color: Some(HexColor {
                    r: 178,
                    g: 34,
                    b: 34,
                    a: 255,
                }),
            }),
            "HTMLRedDarkRed" => Ok(Self {
                color: Some(HexColor {
                    r: 139,
                    g: 0,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLRedRed" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLOrangeOrangeRed" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 69,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLOrangeTomato" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 99,
                    b: 71,
                    a: 255,
                }),
            }),
            "HTMLOrangeCoral" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 127,
                    b: 80,
                    a: 255,
                }),
            }),
            "HTMLOrangeDarkOrange" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 140,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLOrangeOrange" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 165,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLYellowYellow" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 255,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLYellowLightYellow" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 255,
                    b: 224,
                    a: 255,
                }),
            }),
            "HTMLYellowLemonChiffon" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 250,
                    b: 205,
                    a: 255,
                }),
            }),
            "HTMLYellowLightGoldenrodYellow" => Ok(Self {
                color: Some(HexColor {
                    r: 250,
                    g: 250,
                    b: 210,
                    a: 255,
                }),
            }),
            "HTMLYellowPapayaWhip" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 239,
                    b: 213,
                    a: 255,
                }),
            }),
            "HTMLYellowMoccasin" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 228,
                    b: 181,
                    a: 255,
                }),
            }),
            "HTMLYellowPeachPuff" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 218,
                    b: 185,
                    a: 255,
                }),
            }),
            "HTMLYellowPaleGoldenrod" => Ok(Self {
                color: Some(HexColor {
                    r: 238,
                    g: 232,
                    b: 170,
                    a: 255,
                }),
            }),
            "HTMLYellowKhaki" => Ok(Self {
                color: Some(HexColor {
                    r: 240,
                    g: 230,
                    b: 140,
                    a: 255,
                }),
            }),
            "HTMLYellowDarkKhaki" => Ok(Self {
                color: Some(HexColor {
                    r: 189,
                    g: 183,
                    b: 107,
                    a: 255,
                }),
            }),
            "HTMLYellowGold" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 215,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLBrownCornsilk" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 248,
                    b: 220,
                    a: 255,
                }),
            }),
            "HTMLBrownBlanchedAlmond" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 235,
                    b: 205,
                    a: 255,
                }),
            }),
            "HTMLBrownBisque" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 228,
                    b: 196,
                    a: 255,
                }),
            }),
            "HTMLBrownNavajoWhite" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 222,
                    b: 173,
                    a: 255,
                }),
            }),
            "HTMLBrownWheat" => Ok(Self {
                color: Some(HexColor {
                    r: 245,
                    g: 222,
                    b: 179,
                    a: 255,
                }),
            }),
            "HTMLBrownBurlywood" => Ok(Self {
                color: Some(HexColor {
                    r: 222,
                    g: 184,
                    b: 135,
                    a: 255,
                }),
            }),
            "HTMLBrownTan" => Ok(Self {
                color: Some(HexColor {
                    r: 210,
                    g: 180,
                    b: 140,
                    a: 255,
                }),
            }),
            "HTMLBrownRosyBrown" => Ok(Self {
                color: Some(HexColor {
                    r: 188,
                    g: 143,
                    b: 143,
                    a: 255,
                }),
            }),
            "HTMLBrownSandyBrown" => Ok(Self {
                color: Some(HexColor {
                    r: 244,
                    g: 164,
                    b: 96,
                    a: 255,
                }),
            }),
            "HTMLBrownGoldenrod" => Ok(Self {
                color: Some(HexColor {
                    r: 218,
                    g: 165,
                    b: 32,
                    a: 255,
                }),
            }),
            "HTMLBrownDarkGoldenrod" => Ok(Self {
                color: Some(HexColor {
                    r: 184,
                    g: 134,
                    b: 11,
                    a: 255,
                }),
            }),
            "HTMLBrownPeru" => Ok(Self {
                color: Some(HexColor {
                    r: 205,
                    g: 133,
                    b: 63,
                    a: 255,
                }),
            }),
            "HTMLBrownChocolate" => Ok(Self {
                color: Some(HexColor {
                    r: 210,
                    g: 105,
                    b: 30,
                    a: 255,
                }),
            }),
            "HTMLBrownSaddleBrown" => Ok(Self {
                color: Some(HexColor {
                    r: 139,
                    g: 69,
                    b: 19,
                    a: 255,
                }),
            }),
            "HTMLBrownSienna" => Ok(Self {
                color: Some(HexColor {
                    r: 160,
                    g: 82,
                    b: 45,
                    a: 255,
                }),
            }),
            "HTMLBrownBrown" => Ok(Self {
                color: Some(HexColor {
                    r: 165,
                    g: 42,
                    b: 42,
                    a: 255,
                }),
            }),
            "HTMLBrownMaroon" => Ok(Self {
                color: Some(HexColor {
                    r: 128,
                    g: 0,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLGreenDarkOliveGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 85,
                    g: 107,
                    b: 47,
                    a: 255,
                }),
            }),
            "HTMLGreenOlive" => Ok(Self {
                color: Some(HexColor {
                    r: 128,
                    g: 128,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLGreenOliveDrab" => Ok(Self {
                color: Some(HexColor {
                    r: 107,
                    g: 142,
                    b: 35,
                    a: 255,
                }),
            }),
            "HTMLGreenYellowGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 154,
                    g: 205,
                    b: 50,
                    a: 255,
                }),
            }),
            "HTMLGreenLimeGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 50,
                    g: 205,
                    b: 50,
                    a: 255,
                }),
            }),
            "HTMLGreenLime" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 255,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLGreenLawnGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 124,
                    g: 252,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLGreenChartreuse" => Ok(Self {
                color: Some(HexColor {
                    r: 127,
                    g: 255,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLGreenGreenYellow" => Ok(Self {
                color: Some(HexColor {
                    r: 173,
                    g: 255,
                    b: 47,
                    a: 255,
                }),
            }),
            "HTMLGreenSpringGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 255,
                    b: 127,
                    a: 255,
                }),
            }),
            "HTMLGreenMediumSpringGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 250,
                    b: 154,
                    a: 255,
                }),
            }),
            "HTMLGreenLightGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 144,
                    g: 238,
                    b: 144,
                    a: 255,
                }),
            }),
            "HTMLGreenPaleGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 152,
                    g: 251,
                    b: 152,
                    a: 255,
                }),
            }),
            "HTMLGreenDarkSeaGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 143,
                    g: 188,
                    b: 143,
                    a: 255,
                }),
            }),
            "HTMLGreenMediumAquamarine" => Ok(Self {
                color: Some(HexColor {
                    r: 102,
                    g: 205,
                    b: 170,
                    a: 255,
                }),
            }),
            "HTMLGreenMediumSeaGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 60,
                    g: 179,
                    b: 113,
                    a: 255,
                }),
            }),
            "HTMLGreenSeaGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 46,
                    g: 139,
                    b: 87,
                    a: 255,
                }),
            }),
            "HTMLGreenForestGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 34,
                    g: 139,
                    b: 34,
                    a: 255,
                }),
            }),
            "HTMLGreenGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 128,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLGreenDarkGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 100,
                    b: 0,
                    a: 255,
                }),
            }),
            "HTMLCyanAqua" | "HTMLCyanCyan" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 255,
                    b: 255,
                    a: 255,
                }),
            }),
            "HTMLCyanLightCyan" => Ok(Self {
                color: Some(HexColor {
                    r: 224,
                    g: 255,
                    b: 255,
                    a: 255,
                }),
            }),
            "HTMLCyanPaleTurquoise" => Ok(Self {
                color: Some(HexColor {
                    r: 175,
                    g: 238,
                    b: 238,
                    a: 255,
                }),
            }),
            "HTMLCyanAquamarine" => Ok(Self {
                color: Some(HexColor {
                    r: 127,
                    g: 255,
                    b: 212,
                    a: 255,
                }),
            }),
            "HTMLCyanTurquoise" => Ok(Self {
                color: Some(HexColor {
                    r: 64,
                    g: 224,
                    b: 208,
                    a: 255,
                }),
            }),
            "HTMLCyanMediumTurquoise" => Ok(Self {
                color: Some(HexColor {
                    r: 72,
                    g: 209,
                    b: 204,
                    a: 255,
                }),
            }),
            "HTMLCyanDarkTurquoise" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 206,
                    b: 209,
                    a: 255,
                }),
            }),
            "HTMLCyanLightSeaGreen" => Ok(Self {
                color: Some(HexColor {
                    r: 32,
                    g: 178,
                    b: 170,
                    a: 255,
                }),
            }),
            "HTMLCyanCadetBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 95,
                    g: 158,
                    b: 160,
                    a: 255,
                }),
            }),
            "HTMLCyanDarkCyan" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 139,
                    b: 139,
                    a: 255,
                }),
            }),
            "HTMLCyanTeal" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 128,
                    b: 128,
                    a: 255,
                }),
            }),
            "HTMLBlueLightSteelBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 176,
                    g: 196,
                    b: 222,
                    a: 255,
                }),
            }),
            "HTMLBluePowderBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 176,
                    g: 224,
                    b: 230,
                    a: 255,
                }),
            }),
            "HTMLBlueLightBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 173,
                    g: 216,
                    b: 230,
                    a: 255,
                }),
            }),
            "HTMLBlueSkyBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 135,
                    g: 206,
                    b: 235,
                    a: 255,
                }),
            }),
            "HTMLBlueLightSkyBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 135,
                    g: 206,
                    b: 250,
                    a: 255,
                }),
            }),
            "HTMLBlueDeepSkyBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 191,
                    b: 255,
                    a: 255,
                }),
            }),
            "HTMLBlueDodgerBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 30,
                    g: 144,
                    b: 255,
                    a: 255,
                }),
            }),
            "HTMLBlueCornflowerBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 100,
                    g: 149,
                    b: 237,
                    a: 255,
                }),
            }),
            "HTMLBlueSteelBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 70,
                    g: 130,
                    b: 180,
                    a: 255,
                }),
            }),
            "HTMLBlueRoyalBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 65,
                    g: 105,
                    b: 225,
                    a: 255,
                }),
            }),
            "HTMLBlueBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 255,
                }),
            }),
            "HTMLBlueMediumBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 0,
                    b: 205,
                    a: 255,
                }),
            }),
            "HTMLBlueDarkBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 0,
                    b: 139,
                    a: 255,
                }),
            }),
            "HTMLBlueNavy" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 0,
                    b: 128,
                    a: 255,
                }),
            }),
            "HTMLBlueMidnightBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 25,
                    g: 25,
                    b: 112,
                    a: 255,
                }),
            }),
            "HTMLPurpleLavender" => Ok(Self {
                color: Some(HexColor {
                    r: 230,
                    g: 230,
                    b: 250,
                    a: 255,
                }),
            }),
            "HTMLPurpleThistle" => Ok(Self {
                color: Some(HexColor {
                    r: 216,
                    g: 191,
                    b: 216,
                    a: 255,
                }),
            }),
            "HTMLPurplePlum" => Ok(Self {
                color: Some(HexColor {
                    r: 221,
                    g: 160,
                    b: 221,
                    a: 255,
                }),
            }),
            "HTMLPurpleViolet" => Ok(Self {
                color: Some(HexColor {
                    r: 238,
                    g: 130,
                    b: 238,
                    a: 255,
                }),
            }),
            "HTMLPurpleOrchid" => Ok(Self {
                color: Some(HexColor {
                    r: 218,
                    g: 112,
                    b: 214,
                    a: 255,
                }),
            }),
            "HTMLPurpleFuchsia" | "HTMLPurpleMagenta" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 0,
                    b: 255,
                    a: 255,
                }),
            }),
            "HTMLPurpleMediumOrchid" => Ok(Self {
                color: Some(HexColor {
                    r: 186,
                    g: 85,
                    b: 211,
                    a: 255,
                }),
            }),
            "HTMLPurpleMediumPurple" => Ok(Self {
                color: Some(HexColor {
                    r: 147,
                    g: 112,
                    b: 219,
                    a: 255,
                }),
            }),
            "HTMLPurpleBlueViolet" => Ok(Self {
                color: Some(HexColor {
                    r: 138,
                    g: 43,
                    b: 226,
                    a: 255,
                }),
            }),
            "HTMLPurpleDarkViolet" => Ok(Self {
                color: Some(HexColor {
                    r: 148,
                    g: 0,
                    b: 211,
                    a: 255,
                }),
            }),
            "HTMLPurpleDarkOrchid" => Ok(Self {
                color: Some(HexColor {
                    r: 153,
                    g: 50,
                    b: 204,
                    a: 255,
                }),
            }),
            "HTMLPurpleDarkMagenta" => Ok(Self {
                color: Some(HexColor {
                    r: 139,
                    g: 0,
                    b: 139,
                    a: 255,
                }),
            }),
            "HTMLPurplePurple" => Ok(Self {
                color: Some(HexColor {
                    r: 128,
                    g: 0,
                    b: 128,
                    a: 255,
                }),
            }),
            "HTMLPurpleIndigo" => Ok(Self {
                color: Some(HexColor {
                    r: 75,
                    g: 0,
                    b: 130,
                    a: 255,
                }),
            }),
            "HTMLPurpleDarkSlateBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 72,
                    g: 61,
                    b: 139,
                    a: 255,
                }),
            }),
            "HTMLPurpleSlateBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 106,
                    g: 90,
                    b: 205,
                    a: 255,
                }),
            }),
            "HTMLPurpleMediumSlateBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 123,
                    g: 104,
                    b: 238,
                    a: 255,
                }),
            }),
            "HTMLWhiteWhite" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                }),
            }),
            "HTMLWhiteSnow" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 250,
                    b: 250,
                    a: 255,
                }),
            }),
            "HTMLWhiteHoneydew" => Ok(Self {
                color: Some(HexColor {
                    r: 240,
                    g: 255,
                    b: 240,
                    a: 255,
                }),
            }),
            "HTMLWhiteMintCream" => Ok(Self {
                color: Some(HexColor {
                    r: 245,
                    g: 255,
                    b: 250,
                    a: 255,
                }),
            }),
            "HTMLWhiteAzure" => Ok(Self {
                color: Some(HexColor {
                    r: 240,
                    g: 255,
                    b: 255,
                    a: 255,
                }),
            }),
            "HTMLWhiteAliceBlue" => Ok(Self {
                color: Some(HexColor {
                    r: 240,
                    g: 248,
                    b: 255,
                    a: 255,
                }),
            }),
            "HTMLWhiteGhostWhite" => Ok(Self {
                color: Some(HexColor {
                    r: 248,
                    g: 248,
                    b: 255,
                    a: 255,
                }),
            }),
            "HTMLWhiteWhiteSmoke" => Ok(Self {
                color: Some(HexColor {
                    r: 245,
                    g: 245,
                    b: 245,
                    a: 255,
                }),
            }),
            "HTMLWhiteSeashell" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 245,
                    b: 238,
                    a: 255,
                }),
            }),
            "HTMLWhiteBeige" => Ok(Self {
                color: Some(HexColor {
                    r: 245,
                    g: 245,
                    b: 220,
                    a: 255,
                }),
            }),
            "HTMLWhiteOldLace" => Ok(Self {
                color: Some(HexColor {
                    r: 253,
                    g: 245,
                    b: 230,
                    a: 255,
                }),
            }),
            "HTMLWhiteFloralWhite" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 250,
                    b: 240,
                    a: 255,
                }),
            }),
            "HTMLWhiteIvory" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 255,
                    b: 240,
                    a: 255,
                }),
            }),
            "HTMLWhiteAntiqueWhite" => Ok(Self {
                color: Some(HexColor {
                    r: 250,
                    g: 235,
                    b: 215,
                    a: 255,
                }),
            }),
            "HTMLWhiteLinen" => Ok(Self {
                color: Some(HexColor {
                    r: 250,
                    g: 240,
                    b: 230,
                    a: 255,
                }),
            }),
            "HTMLWhiteLavenderBlush" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 240,
                    b: 245,
                    a: 255,
                }),
            }),
            "HTMLWhiteMistyRose" => Ok(Self {
                color: Some(HexColor {
                    r: 255,
                    g: 228,
                    b: 225,
                    a: 255,
                }),
            }),
            "HTMLGrayGainsboro" => Ok(Self {
                color: Some(HexColor {
                    r: 220,
                    g: 220,
                    b: 220,
                    a: 255,
                }),
            }),
            "HTMLGrayLightGray" => Ok(Self {
                color: Some(HexColor {
                    r: 211,
                    g: 211,
                    b: 211,
                    a: 255,
                }),
            }),
            "HTMLGraySilver" => Ok(Self {
                color: Some(HexColor {
                    r: 192,
                    g: 192,
                    b: 192,
                    a: 255,
                }),
            }),
            "HTMLGrayDarkGray" => Ok(Self {
                color: Some(HexColor {
                    r: 169,
                    g: 169,
                    b: 169,
                    a: 255,
                }),
            }),
            "HTMLGrayGray" => Ok(Self {
                color: Some(HexColor {
                    r: 128,
                    g: 128,
                    b: 128,
                    a: 255,
                }),
            }),
            "HTMLGrayDimGray" => Ok(Self {
                color: Some(HexColor {
                    r: 105,
                    g: 105,
                    b: 105,
                    a: 255,
                }),
            }),
            "HTMLGrayLightSlateGray" => Ok(Self {
                color: Some(HexColor {
                    r: 119,
                    g: 136,
                    b: 153,
                    a: 255,
                }),
            }),
            "HTMLGraySlateGray" => Ok(Self {
                color: Some(HexColor {
                    r: 112,
                    g: 128,
                    b: 144,
                    a: 255,
                }),
            }),
            "HTMLGrayDarkSlateGray" => Ok(Self {
                color: Some(HexColor {
                    r: 47,
                    g: 79,
                    b: 79,
                    a: 255,
                }),
            }),
            "HTMLGrayBlack" => Ok(Self {
                color: Some(HexColor {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                }),
            }),
            _ => Err("Not a valid option"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct StyleData {
    pub line_style: LineStyle,
    pub line_weight: LineWeight,
    pub fill_color: QETColor,
    pub line_color: QETColor,
}

impl Default for StyleData {
    fn default() -> Self {
        Self {
            line_style: LineStyle::default(),
            line_weight: LineWeight::default(),
            fill_color: QETColor::default(),
            line_color: QETColor {
                color: Some(HexColor::rgb(0, 0, 0)),
            },
        }
    }
}

//Style Strings
//"line-style:normal;line-weight:normal;filling:none;color:black"
//"line-style:normal;line-weight:thin;filling:none;color:black"
impl Display for StyleData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            line_style,
            line_weight,
            fill_color,
            line_color,
        } = self;
        write!(f, "line-style:{line_style};line-weight:{line_weight};filling:{fill_color};color:{line_color}")
    }
}
