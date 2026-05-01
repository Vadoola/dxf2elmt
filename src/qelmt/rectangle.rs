use crate::qelmt::style::{LineWeight, StyleData};

use super::{two_dec, Bounding, Rectangularity, ScaleEntity};
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

impl TryFrom<&Polyline> for Rectangle {
    type Error = &'static str; //add better error type later

    fn try_from(poly: &Polyline) -> Result<Self, Self::Error> {
        if !poly.is_rectangular() {
            return Err("Polyline does not appear to be rectangular, can't convert");
        }

        Ok(Rectangle {
            x: poly.left_bound(),
            y: -poly.top_bound(),
            height: (poly.bot_bound() - poly.top_bound()).abs(),
            width: (poly.right_bound() - poly.left_bound()).abs(),
            rx: 0.0,
            ry: 0.0,
            antialias: false,
            style: StyleData {
                line_weight: LineWeight::Thin,
                ..Default::default()
            },
        })
    }
}

impl TryFrom<&LwPolyline> for Rectangle {
    type Error = &'static str; //add better error type later

    fn try_from(poly: &LwPolyline) -> Result<Self, Self::Error> {
        if !poly.is_rectangular() {
            return Err("LwPolyline does not appear to be rectangular, can't convert");
        }

        Ok(Rectangle {
            x: poly.left_bound(),
            y: -poly.top_bound(),
            height: (poly.bot_bound() - poly.top_bound()).abs(),
            width: (poly.right_bound() - poly.left_bound()).abs(),
            rx: 0.0,
            ry: 0.0,
            antialias: false,
            style: StyleData {
                line_weight: LineWeight::Thin,
                ..Default::default()
            },
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
