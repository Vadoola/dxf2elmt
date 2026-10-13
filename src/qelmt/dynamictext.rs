use crate::qelmt::Bounding;

use super::{two_dec, FontInfo, ScaleEntity, TextEntity};
use dxf::entities::{self, AttributeDefinition};
use hex_color::HexColor;
use simple_xml_builder::XMLElement;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

/*use parley::{
    Alignment, FontContext, FontWeight, InlineBox, Layout, LayoutContext, PositionedLayoutItem,
    StyleProperty,
};*/

use super::{HAlignment, VAlignment};

#[derive(Debug)]
pub struct DynamicText {
    text: String,
    info_name: Option<String>,
    pub x: f64,
    pub y: f64,
    z: f64,
    rotation: f64,
    uuid: Uuid,
    h_alignment: HAlignment,
    font: FontInfo,
    text_from: String,
    v_alignment: VAlignment,
    frame: bool,
    text_width: i32,
    keep_visual_rotation: bool,
    color: HexColor,
    reference_rectangle_width: f64,
}

impl From<&DynamicText> for XMLElement {
    fn from(txt: &DynamicText) -> Self {
        let mut dtxt_xml = XMLElement::new("dynamic_text");
        // taken from QET_ElementScaler: "ElmtDynText::AsSVGstring"
        //    // Position und Rotationspunkt berechnen:
        //    posx = x + (size/8.0)+4.05 - 0.5;
        //    posy = y + (7.0/5.0*size + 26.0/5.0) - 0.5;
        //    rotx = (-1) * (((size/8.0)+4.05) - 0.5);
        //    roty = (-1) * ((7.0/5.0*size + 26.0/5.0) - 0.5);
        //
        // reversed and slightly modified after looking at the result in element-editor:
        //
        let pt_size: f64 = txt.font.point_size;
        //
        // we need the horizontal alignment and the text-width to move to right x-position:
        // txt.reference_rectangle_width, // should be text-width (Group code 41)
        // txt.attachment_point,  // Group code 71
        //                        // 1 = Top left; 2 = Top center; 3 = Top right
        //                        // 4 = Middle left; 5 = Middle center; 6 = Middle right
        //                        // 7 = Bottom left; 8 = Bottom center; 9 = Bottom right
        //
        //
        // it's just annoying if the value for "reference_rectangle_width" in the dxf is “0.0”...
        //
        // o.k. ... as long as we do not know the real width:
        // "guess" the width by number of characters and font-size:
        //
        let graphene_count = txt.text.graphemes(true).count();
        let txt_width = if txt.reference_rectangle_width > 2.0 {
            txt.reference_rectangle_width
        } else {
            (graphene_count as f64) * pt_size * 0.75
        };

        let x_pos = {
            let x_pos = txt.x + 0.5 - (pt_size / 8.0) - 4.05;
            match txt.h_alignment {
                HAlignment::Left => x_pos,
                HAlignment::Center => x_pos - txt_width / 2.0,
                HAlignment::Right => x_pos - txt_width,
            }
        };
        let y_pos = txt.y + 0.5 - (7.0 / 5.0 * pt_size + 26.0 / 5.0) + pt_size;

        dtxt_xml.add_attribute("x", two_dec(x_pos));
        dtxt_xml.add_attribute("y", two_dec(y_pos));
        dtxt_xml.add_attribute("z", two_dec(txt.z));
        dtxt_xml.add_attribute("rotation", two_dec(txt.rotation));
        dtxt_xml.add_attribute("uuid", format!("{{{}}}", txt.uuid));
        dtxt_xml.add_attribute("font", &txt.font);
        dtxt_xml.add_attribute("Halignment", &txt.h_alignment);
        dtxt_xml.add_attribute("Valignment", &txt.v_alignment);
        dtxt_xml.add_attribute("text_from", &txt.text_from);
        dtxt_xml.add_attribute("frame", txt.frame);
        dtxt_xml.add_attribute("text_width", txt.text_width);
        dtxt_xml.add_attribute("color", txt.color.display_rgb());

        //If I ever add support for other text_from types, element and composite text
        //I'll need to add more smarts here, as there may be some other children components
        //for now since it only supports user_text I'm just statically adding the single child
        //component needed
        //match txt.text_from
        let mut text_xml = XMLElement::new("text");
        text_xml.add_text(&txt.text);
        dtxt_xml.add_child(text_xml);

        if let Some(i_name) = &txt.info_name {
            dtxt_xml.add_attribute("info_name", i_name);
        }

        if txt.keep_visual_rotation {
            dtxt_xml.add_attribute("keep_visual_rotation", txt.keep_visual_rotation);
        }

        dtxt_xml
    }
}

impl Bounding for DynamicText {
    fn left_bound(&self) -> f64 {
        self.x
    }

    fn right_bound(&self) -> f64 {
        //todo!()
        1.0
    }

    fn top_bound(&self) -> f64 {
        self.y
    }

    fn bot_bound(&self) -> f64 {
        //todo!()
        1.0
    }
}

impl ScaleEntity for DynamicText {
    fn scale(&mut self, fact_x: f64, fact_y: f64) {
        self.x *= fact_x;
        self.y *= fact_y;
        //self.font.pixel_size *= fact;
        self.font.point_size *= fact_x;
    }
}

pub struct DTextBuilder<'a> {
    text: TextEntity<'a>,
    color: Option<HexColor>,
}

impl<'a> DTextBuilder<'a> {
    pub fn from_text(text: &'a entities::Text) -> Self {
        Self {
            text: TextEntity::Text(text),
            color: None,
        }
    }

    pub fn from_mtext(text: &'a entities::MText) -> Self {
        Self {
            text: TextEntity::MText(text),
            color: None,
        }
    }

    pub fn from_attrib(attrib: &'a AttributeDefinition) -> Self {
        Self {
            text: TextEntity::Attrib(attrib),
            color: None,
        }
    }

    pub fn color(self, color: HexColor) -> Self {
        Self {
            color: Some(color),
            ..self
        }
    }

    pub fn build(self) -> DynamicText {
        let (
            x,
            y,
            z,
            rotation,
            style_name,
            text_height,
            value,
            h_alignment,
            v_alignment,
            reference_rectangle_width,
        ) = match self.text {
            TextEntity::Text(txt) => (
                txt.location.x,
                -txt.location.y,
                txt.location.z,
                txt.rotation,
                &txt.text_style_name,
                txt.text_height,
                txt.value.clone(),
                HAlignment::from(txt.horizontal_text_justification),
                VAlignment::from(txt.vertical_text_justification),
                0.0, // as Placeholder: no "reference_rectangle_width" with Text!!!
            ),
            TextEntity::MText(mtxt) => (
                mtxt.insertion_point.x,
                -mtxt.insertion_point.y,
                mtxt.insertion_point.z,
                mtxt.rotation_angle,
                &mtxt.text_style_name,
                //I'm not sure what the proper value is here for Mtext
                //becuase I haven't actually finished supporting it.
                //I'll put initial text height for now. But i'm not certain
                //exactly what this correlates to. There is also vertical_height,
                //which I would guess is the total vertical height for all the lines
                //it's possible I would need to take the vertical height and divide
                //by the number of lines to get the value I need....I'm not sure yet
                mtxt.initial_text_height,
                //There are 2 text fields on MTEXT, .text a String and .extended_text a Vec<String>
                //Most of the example files I have at the moment are single line MTEXT.
                //I edited one of them in QCad, and added a few lines. The value came through in the text field
                //with extended_text being empty, and the newlines were deliniated by '\\P'...I might need to look
                //the spec a bit to determine what it says for MTEXT, but for now, I'll just assume this is correct
                //So looking at the spec, yes '\P' is the MTEXT newline essentially. There is a bunch of MTEXT
                //inline codes that can be found at https://ezdxf.readthedocs.io/en/stable/dxfentities/mtext.html
                //The extended text is code point 3 in the dxf spec which just says: "Additional text (always in 250-character chunks) (optional)"
                //and Code point 1 the normal text value says: "Text string. If the text string is less than 250 characters, all characters appear
                //in group 1. If the text string is greater than 250 characters, the string is divided into 250-character chunks, which appear in
                //one or more group 3 codes. If group 3 codes are used, the last group is a group 1 and has fewer than 250 characters"
                {
                    let mut val = mtxt.extended_text.join("");
                    val.push_str(&mtxt.text);
                    val.replace("\\P", "\n")
                },
                HAlignment::from(mtxt.attachment_point),
                VAlignment::from(mtxt.attachment_point),
                mtxt.reference_rectangle_width,
            ),
            TextEntity::Attrib(attrib) => (
                attrib.location.x,
                -attrib.location.y,
                attrib.location.z,
                attrib.rotation,
                &attrib.text_style_name,
                attrib.text_height,
                attrib.value.clone(),
                HAlignment::from(attrib.horizontal_text_justification),
                VAlignment::from(attrib.vertical_text_justification),
                0.0, // as Placeholder: not need to check if Attrib has something similar
            ),
        };

        // Create a FontContext (font database) and LayoutContext (scratch space).
        // These are both intended to be constructed rarely (perhaps even once per app):
        /*let mut font_cx = FontContext::new();
        let mut layout_cx = LayoutContext::new();

        // Create a `RangedBuilder` or a `TreeBuilder`, which are used to construct a `Layout`.
        const DISPLAY_SCALE : f32 = 1.0;
        let mut builder = layout_cx.ranged_builder(&mut font_cx, &value, DISPLAY_SCALE);

        // Set default styles that apply to the entire layout
        builder.push_default(StyleProperty::LineHeight(1.3));
        builder.push_default(StyleProperty::FontSize((text_height * self.txt_sc_factor.unwrap()).round() as f32));

        // Build the builder into a Layout
        let mut layout: Layout<()> = builder.build(&value);

        // Run line-breaking and alignment on the Layout
        const MAX_WIDTH : Option<f32> = Some(1000.0);
        layout.break_all_lines(MAX_WIDTH);
        layout.align(MAX_WIDTH, Alignment::Start);

        let calc_width = layout.width();
        let calc_height = layout.height();
        dbg!(&value);
        dbg!(calc_width);
        dbg!(calc_height);*/

        /*dbg!(&value);
        dbg!(&y);
        dbg!(&self.text);*/
        DynamicText {
            //x: x - (calc_width as f64/2.0),
            x,
            y,
            z,
            rotation: if rotation.abs().round() as i64 % 360 != 0 {
                rotation - 180.0
            } else {
                0.0
            },
            uuid: Uuid::new_v4(),
            font: if style_name == "STANDARD" {
                FontInfo {
                    point_size: text_height,
                    ..Default::default()
                }
            } else {
                //clearly right now this is exactly the same as the main body of the if block
                //I'm jus putting this in for now, to compile while I get the font handling
                //working correctly
                FontInfo {
                    point_size: text_height,
                    ..Default::default()
                }
            },
            reference_rectangle_width, //liest aus der dxf-Datei!!!
            h_alignment,
            v_alignment,
            text_from: "UserText".into(),
            frame: false,
            text_width: -1,
            color: self.color.unwrap_or(HexColor::BLACK),

            text: value,
            keep_visual_rotation: false,
            info_name: None,
        }
    }
}
