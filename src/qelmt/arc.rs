use super::{two_dec, ScaleEntity};
use dxf::entities;
use simple_xml_builder::XMLElement;

#[derive(Debug)]
pub struct Arc {
    //need to brush up on my Rust scoping rules, isn't there a way to make this pub to just the module?
    pub x: f64,
    pub y: f64,

    width: f64,
    height: f64,
    start: f64,
    angle: f64,
    style: String,
    antialias: bool,
}

impl From<&entities::Arc> for Arc {
    fn from(arc: &entities::Arc) -> Self {
        let temp_angle = if arc.start_angle > arc.end_angle {
            (360.0 - arc.start_angle) + arc.end_angle
        } else {
            arc.end_angle - arc.start_angle
        };

        Arc {
            x: arc.center.x - arc.radius,
            y: -arc.center.y - arc.radius,
            height: arc.radius * 2.0,
            width: arc.radius * 2.0,
            start: if arc.start_angle < 0.0 {
                -arc.start_angle
            } else {
                arc.start_angle
            },
            angle: if temp_angle < 0.0 {
                -temp_angle
            } else {
                temp_angle
            },

            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: if arc.thickness > 0.1 {
                "line-style:normal;line-weight:normal;filling:none;color:black"
            } else {
                "line-style:normal;line-weight:thin;filling:none;color:black"
            }
            .into(),
        }
    }
}

impl From<&Arc> for XMLElement {
    fn from(arc: &Arc) -> Self {
        let mut arc_xml: XMLElement = XMLElement::new("arc");
        arc_xml.add_attribute("x", two_dec(arc.x));
        arc_xml.add_attribute("y", two_dec(arc.y));
        arc_xml.add_attribute("width", two_dec(arc.width));
        arc_xml.add_attribute("height", two_dec(arc.height));
        arc_xml.add_attribute("start", arc.start.round());
        arc_xml.add_attribute("angle", arc.angle.round());
        arc_xml.add_attribute("antialias", arc.antialias);
        arc_xml.add_attribute("style", &arc.style);
        arc_xml
    }
}

impl ScaleEntity for Arc {
    fn scale(&mut self, fact_x: f64, fact_y: f64) {
        self.x *= fact_x;
        self.y *= fact_y;
        self.width *= fact_x;
        self.height *= fact_y;
    }
}