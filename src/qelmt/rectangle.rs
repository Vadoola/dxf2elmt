use crate::qelmt::style::{LineWeight, StyleData};

use super::{Bounding, Rectangularity, ScaleEntity, two_dec};
use dxf::entities::{LwPolyline, Polyline};
use simple_xml_builder::XMLElement;

#[derive(Debug)]
pub struct Rectangle {
    height: f64,
    width: f64,
    style: StyleData,

    //need to brush up on my Rust scoping rules, isn't there a way to make this pub to just the module?
    pub x: f64,
    pub y: f64,

    rx: f64,
    ry: f64,

    antialias: bool,
}

enum RectSource<'a> {
    Polyline(&'a Polyline),
    LwPolyline(&'a LwPolyline),
}

pub struct RectBuilder<'a> {
    source: RectSource<'a>,
    style: Option<StyleData>,
    antialias: bool,
}

impl<'a> RectBuilder<'a> {
    pub fn from_polyline(poly: &'a Polyline) -> Self {
        Self {
            source: RectSource::Polyline(poly),
            style: None,
            antialias: false,
        }
    }

    pub fn from_lwpolyline(poly: &'a LwPolyline) -> Self {
        Self {
            source: RectSource::LwPolyline(poly),
            style: None,
            antialias: false,
        }
    }

    pub fn style(self, style: StyleData) -> Self {
        Self {
            style: Some(style),
            ..self
        }
    }

    pub fn antialias(self, antialias: bool) -> Self {
        Self {
            antialias,
            ..self
        }
    }

    pub fn build(self) -> Result<Rectangle, &'static str /*TODO: add better error type later*/> {
        Ok(match self.source {
            RectSource::Polyline(poly) => {
                if !poly.is_rectangular() {
                    return Err("Polyline does not appear to be rectangular, can't convert");
                }

                Rectangle {
                    x: poly.left_bound(),
                    y: -poly.top_bound(),
                    height: (poly.bot_bound() - poly.top_bound()).abs(),
                    width: (poly.right_bound() - poly.left_bound()).abs(),
                    rx: 0.0,
                    ry: 0.0,
                    antialias: self.antialias,
                    style: StyleData {
                        line_weight: LineWeight::Thin,
                        ..self.style.unwrap_or_default()
                    },
                }
            }
            RectSource::LwPolyline(lwpoly) => {
                if !lwpoly.is_rectangular() {
                    return Err("LwPolyline does not appear to be rectangular, can't convert");
                }

                Rectangle {
                    x: lwpoly.left_bound(),
                    y: -lwpoly.top_bound(),
                    height: (lwpoly.bot_bound() - lwpoly.top_bound()).abs(),
                    width: (lwpoly.right_bound() - lwpoly.left_bound()).abs(),
                    rx: 0.0,
                    ry: 0.0,
                    antialias: self.antialias,
                    style: StyleData {
                        line_weight: LineWeight::Thin,
                        ..self.style.unwrap_or_default()
                    },
                }
            }
        })
    }
}

impl From<&Rectangle> for XMLElement {
    fn from(rec: &Rectangle) -> Self {
        let mut rec_xml = XMLElement::new("rect");
        rec_xml.add_attribute("x", two_dec(rec.x));
        rec_xml.add_attribute("y", two_dec(rec.y));
        rec_xml.add_attribute("rx", two_dec(rec.rx));
        rec_xml.add_attribute("ry", two_dec(rec.ry));
        rec_xml.add_attribute("height", two_dec(rec.height));
        rec_xml.add_attribute("width", two_dec(rec.width));
        rec_xml.add_attribute("antialias", rec.antialias);
        rec_xml.add_attribute("style", &rec.style);
        rec_xml
    }
}

impl Bounding for Rectangle {
    fn left_bound(&self) -> f64 {
        self.x
    }

    fn right_bound(&self) -> f64 {
        self.x + self.width
    }

    fn top_bound(&self) -> f64 {
        self.y
    }

    fn bot_bound(&self) -> f64 {
        self.y + self.height
    }
}

impl ScaleEntity for Rectangle {
    fn scale(&mut self, fact_x: f64, fact_y: f64) {
        self.x *= fact_x;
        self.y *= fact_y;
        self.width *= fact_x;
        self.height *= fact_y;
        // should I be scaling the corner radii?
        // right now they will default to 0, and unless
        // I come up with some way to determine a rounded rectangle
        // vs a regular rectangle made of polylines, I'm not sure if
        // I will ever actually use the corner radii
    }
}
