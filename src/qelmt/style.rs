use std::fmt::Display;

use hex_color::HexColor;
//Style Strings
//"line-style:normal;line-weight:normal;filling:none;color:black"
//"line-style:normal;line-weight:thin;filling:none;color:black"

enum LineStyle {
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

enum LineWeight {
    None,
    Thin,
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

/*
{"yellow", Qt::yellow},
{"cyan", Qt::cyan},
{"magenta", Qt::magenta},
{"lightgray", Qt::lightGray},
{"orange", QColor(255, 128, 0)},
{"purple", QColor(136, 28, 168)},
{"HTMLPinkPink", QColor(255, 192, 203)},
{"HTMLPinkLightPink", QColor(255, 182, 193)},
{"HTMLPinkHotPink", QColor(255, 105, 180)},
{"HTMLPinkDeepPink", QColor(255, 20, 147)},
{"HTMLPinkPaleVioletRed", QColor(219, 112, 147)},
{"HTMLPinkMediumVioletRed", QColor(199, 21, 133)},
{"HTMLRedLightSalmon", QColor(255, 160, 122)},
{"HTMLRedSalmon", QColor(250, 128, 114)},
{"HTMLRedDarkSalmon", QColor(233, 150, 122)},
{"HTMLRedLightCoral", QColor(240, 128, 128)},
{"HTMLRedIndianRed", QColor(205, 92, 92)},
{"HTMLRedCrimson", QColor(220, 20, 60)},
{"HTMLRedFirebrick", QColor(178, 34, 34)},
{"HTMLRedDarkRed", QColor(139, 0, 0)},
{"HTMLRedRed", QColor(255, 0, 0)},
{"HTMLOrangeOrangeRed", QColor(255, 69, 0)},
{"HTMLOrangeTomato", QColor(255, 99, 71)},
{"HTMLOrangeCoral", QColor(255, 127, 80)},
{"HTMLOrangeDarkOrange", QColor(255, 140, 0)},
{"HTMLOrangeOrange", QColor(255, 165, 0)},
{"HTMLYellowYellow", QColor(255, 255, 0)},
{"HTMLYellowLightYellow", QColor(255, 255, 224)},
{"HTMLYellowLemonChiffon", QColor(255, 250, 205)},
{"HTMLYellowLightGoldenrodYellow", QColor(250, 250, 210)},
{"HTMLYellowPapayaWhip", QColor(255, 239, 213)},
{"HTMLYellowMoccasin", QColor(255, 228, 181)},
{"HTMLYellowPeachPuff", QColor(255, 218, 185)},
{"HTMLYellowPaleGoldenrod", QColor(238, 232, 170)},
{"HTMLYellowKhaki", QColor(240, 230, 140)},
{"HTMLYellowDarkKhaki", QColor(189, 183, 107)},
{"HTMLYellowGold", QColor(255, 215, 0)},
{"HTMLBrownCornsilk", QColor(255, 248, 220)},
{"HTMLBrownBlanchedAlmond", QColor(255, 235, 205)},
{"HTMLBrownBisque", QColor(255, 228, 196)},
{"HTMLBrownNavajoWhite", QColor(255, 222, 173)},
{"HTMLBrownWheat", QColor(245, 222, 179)},
{"HTMLBrownBurlywood", QColor(222, 184, 135)},
{"HTMLBrownTan", QColor(210, 180, 140)},
{"HTMLBrownRosyBrown", QColor(188, 143, 143)},
{"HTMLBrownSandyBrown", QColor(244, 164, 96)},
{"HTMLBrownGoldenrod", QColor(218, 165, 32)},
{"HTMLBrownDarkGoldenrod", QColor(184, 134, 11)},
{"HTMLBrownPeru", QColor(205, 133, 63)},
{"HTMLBrownChocolate", QColor(210, 105, 30)},
{"HTMLBrownSaddleBrown", QColor(139, 69, 19)},
{"HTMLBrownSienna", QColor(160, 82, 45)},
{"HTMLBrownBrown", QColor(165, 42, 42)},
{"HTMLBrownMaroon", QColor(128, 0, 0)},
{"HTMLGreenDarkOliveGreen", QColor(85, 107, 47)},
{"HTMLGreenOlive", QColor(128, 128, 0)},
{"HTMLGreenOliveDrab", QColor(107, 142, 35)},
{"HTMLGreenYellowGreen", QColor(154, 205, 50)},
{"HTMLGreenLimeGreen", QColor(50, 205, 50)},
{"HTMLGreenLime", QColor(0, 255, 0)},
{"HTMLGreenLawnGreen", QColor(124, 252, 0)},
{"HTMLGreenChartreuse", QColor(127, 255, 0)},
{"HTMLGreenGreenYellow", QColor(173, 255, 47)},
{"HTMLGreenSpringGreen", QColor(0, 255, 127)},
{"HTMLGreenMediumSpringGreen", QColor(0, 250, 154)},
{"HTMLGreenLightGreen", QColor(144, 238, 144)},
{"HTMLGreenPaleGreen", QColor(152, 251, 152)},
{"HTMLGreenDarkSeaGreen", QColor(143, 188, 143)},
{"HTMLGreenMediumAquamarine", QColor(102, 205, 170)},
{"HTMLGreenMediumSeaGreen", QColor(60, 179, 113)},
{"HTMLGreenSeaGreen", QColor(46, 139, 87)},
{"HTMLGreenForestGreen", QColor(34, 139, 34)},
{"HTMLGreenGreen", QColor(0, 128, 0)},
{"HTMLGreenDarkGreen", QColor(0, 100, 0)},
{"HTMLCyanAqua", QColor(0, 255, 255)},
{"HTMLCyanCyan", QColor(0, 255, 255)},
{"HTMLCyanLightCyan", QColor(224, 255, 255)},
{"HTMLCyanPaleTurquoise", QColor(175, 238, 238)},
{"HTMLCyanAquamarine", QColor(127, 255, 212)},
{"HTMLCyanTurquoise", QColor(64, 224, 208)},
{"HTMLCyanMediumTurquoise", QColor(72, 209, 204)},
{"HTMLCyanDarkTurquoise", QColor(0, 206, 209)},
{"HTMLCyanLightSeaGreen", QColor(32, 178, 170)},
{"HTMLCyanCadetBlue", QColor(95, 158, 160)},
{"HTMLCyanDarkCyan", QColor(0, 139, 139)},
{"HTMLCyanTeal", QColor(0, 128, 128)},
{"HTMLBlueLightSteelBlue", QColor(176, 196, 222)},
{"HTMLBluePowderBlue", QColor(176, 224, 230)},
{"HTMLBlueLightBlue", QColor(173, 216, 230)},
{"HTMLBlueSkyBlue", QColor(135, 206, 235)},
{"HTMLBlueLightSkyBlue", QColor(135, 206, 250)},
{"HTMLBlueDeepSkyBlue", QColor(0, 191, 255)},
{"HTMLBlueDodgerBlue", QColor(30, 144, 255)},
{"HTMLBlueCornflowerBlue", QColor(100, 149, 237)},
{"HTMLBlueSteelBlue", QColor(70, 130, 180)},
{"HTMLBlueRoyalBlue", QColor(65, 105, 225)},
{"HTMLBlueBlue", QColor(0, 0, 255)},
{"HTMLBlueMediumBlue", QColor(0, 0, 205)},
{"HTMLBlueDarkBlue", QColor(0, 0, 139)},
{"HTMLBlueNavy", QColor(0, 0, 128)},
{"HTMLBlueMidnightBlue", QColor(25, 25, 112)},
{"HTMLPurpleLavender", QColor(230, 230, 250)},
{"HTMLPurpleThistle", QColor(216, 191, 216)},
{"HTMLPurplePlum", QColor(221, 160, 221)},
{"HTMLPurpleViolet", QColor(238, 130, 238)},
{"HTMLPurpleOrchid", QColor(218, 112, 214)},
{"HTMLPurpleFuchsia", QColor(255, 0, 255)},
{"HTMLPurpleMagenta", QColor(255, 0, 255)},
{"HTMLPurpleMediumOrchid", QColor(186, 85, 211)},
{"HTMLPurpleMediumPurple", QColor(147, 112, 219)},
{"HTMLPurpleBlueViolet", QColor(138, 43, 226)},
{"HTMLPurpleDarkViolet", QColor(148, 0, 211)},
{"HTMLPurpleDarkOrchid", QColor(153, 50, 204)},
{"HTMLPurpleDarkMagenta", QColor(139, 0, 139)},
{"HTMLPurplePurple", QColor(128, 0, 128)},
{"HTMLPurpleIndigo", QColor(75, 0, 130)},
{"HTMLPurpleDarkSlateBlue", QColor(72, 61, 139)},
{"HTMLPurpleSlateBlue", QColor(106, 90, 205)},
{"HTMLPurpleMediumSlateBlue", QColor(123, 104, 238)},
{"HTMLWhiteWhite", QColor(255, 255, 255)},
{"HTMLWhiteSnow", QColor(255, 250, 250)},
{"HTMLWhiteHoneydew", QColor(240, 255, 240)},
{"HTMLWhiteMintCream", QColor(245, 255, 250)},
{"HTMLWhiteAzure", QColor(240, 255, 255)},
{"HTMLWhiteAliceBlue", QColor(240, 248, 255)},
{"HTMLWhiteGhostWhite", QColor(248, 248, 255)},
{"HTMLWhiteWhiteSmoke", QColor(245, 245, 245)},
{"HTMLWhiteSeashell", QColor(255, 245, 238)},
{"HTMLWhiteBeige", QColor(245, 245, 220)},
{"HTMLWhiteOldLace", QColor(253, 245, 230)},
{"HTMLWhiteFloralWhite", QColor(255, 250, 240)},
{"HTMLWhiteIvory", QColor(255, 255, 240)},
{"HTMLWhiteAntiqueWhite", QColor(250, 235, 215)},
{"HTMLWhiteLinen", QColor(250, 240, 230)},
{"HTMLWhiteLavenderBlush", QColor(255, 240, 245)},
{"HTMLWhiteMistyRose", QColor(255, 228, 225)},
{"HTMLGrayGainsboro", QColor(220, 220, 220)},
{"HTMLGrayLightGray", QColor(211, 211, 211)},
{"HTMLGraySilver", QColor(192, 192, 192)},
{"HTMLGrayDarkGray", QColor(169, 169, 169)},
{"HTMLGrayGray", QColor(128, 128, 128)},
{"HTMLGrayDimGray", QColor(105, 105, 105)},
{"HTMLGrayLightSlateGray", QColor(119, 136, 153)},
{"HTMLGraySlateGray", QColor(112, 128, 144)},
{"HTMLGrayDarkSlateGray", QColor(47, 79, 79)},
{"HTMLGrayBlack", QColor(0, 0, 0)}*/
#[derive(Default)]
pub struct QETColor {
    color: Option<HexColor>,
}

impl From<HexColor> for QETColor {
    fn from(color: HexColor) -> Self {
        Self { color: Some(color) }
    }
}

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
                        HexColor {r: 0xD9, g: 0x00, b: 0x00, a: _} => "red",
                        HexColor {r: 0x00, g: 0x00, b: 0xD9, a: _} => "blue",
                        HexColor {r: 0x00, g: 0xD9, b: 0x00, a: _} => "green",
                        HexColor {r: 0x88, g: 0x88, b: 0x8D, a: _} => "gray",
                        HexColor {r: 0x61, g: 0x2C, b: 0x00, a: _} => "brun",
                        
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

//Hmm it lokos like these colors aren't actually stored as hex values but
//as a string like so:
//filling:HTMLBrownWheat
//color:blue
pub struct StyleData {
    line_style: LineStyle,
    line_weight: LineWeight,
    fill_color: QETColor,
    line_color: QETColor,
}
