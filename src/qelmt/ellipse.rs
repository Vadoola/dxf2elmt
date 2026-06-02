use crate::qelmt::style::{LineWeight, StyleData};

use super::{Bounding, Circularity, ScaleEntity, two_dec};
use dxf::entities::{self, Circle, LwPolyline, Polyline};
use simple_xml_builder::XMLElement;

#[derive(Debug)]
pub struct Ellipse {
    height: f64,
    width: f64,
    style: StyleData,

    //need to brush up on my Rust scoping rules, isn't there a way to make this pub to just the module?
    pub x: f64,
    pub y: f64,

    antialias: bool,
}

enum EllipSource<'a> {
    Circle(&'a Circle),
    Ellipse(&'a entities::Ellipse),
    Polyline(&'a Polyline),
    LwPolyline(&'a LwPolyline),
}

pub struct EllipBuilder<'a> {
    source: EllipSource<'a>,
    style: Option<StyleData>,
    antialias: bool,
}

impl<'a> EllipBuilder<'a> {
    pub fn from_circle(circ: &'a Circle) -> Self {
        Self {
            source: EllipSource::Circle(circ),
            style: None,
            antialias: false,
        }
    }

    pub fn from_ellipse(ellipse: &'a entities::Ellipse) -> Self {
        Self {
            source: EllipSource::Ellipse(ellipse),
            style: None,
            antialias: false,
        }
    }

    pub fn from_polyline(poly: &'a Polyline) -> Result<Self, &'static str/*TODO: Need Better Error*/> {
        if !poly.is_circular() {
            return Err("Polyline has poor circularity, can't convert");
        }
        
        Ok(Self {
            source: EllipSource::Polyline(poly),
            style: None,
            antialias: false,
        })
    }

    pub fn from_lwpolyline(poly: &'a LwPolyline) -> Result<Self, &'static str/*TODO: Need Better Error*/> {
        if !poly.is_circular() {
            return Err("Polyline has poor circularity, can't convert");
        }

        Ok(Self {
            source: EllipSource::LwPolyline(poly),
            style: None,
            antialias: false,
        })
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

    pub fn build(self) -> Ellipse {
        match self.source {
            EllipSource::Circle(circ) => {
                Ellipse {
                    x: circ.center.x - circ.radius,
                    y: -circ.center.y - circ.radius,
                    height: circ.radius * 2.0,
                    width: circ.radius * 2.0,

                    antialias: self.antialias,
                    style: if circ.thickness > 0.5 {
                        self.style.unwrap_or_default()
                    } else {
                        StyleData {
                            line_weight: LineWeight::Thin,
                            ..self.style.unwrap_or_default()
                        }
                    },
                }
            }
            EllipSource::Ellipse(ellipse) => {
                Ellipse {
                    x: ellipse.center.x - ellipse.major_axis.x,
                    y: -ellipse.center.y - ellipse.major_axis.x * ellipse.minor_axis_ratio,
                    height: ellipse.major_axis.x * 2.0,
                    width: ellipse.major_axis.x * 2.0 * ellipse.minor_axis_ratio,

                    antialias: self.antialias,
                    style: StyleData {
                        line_weight: LineWeight::Thin,
                        ..self.style.unwrap_or_default()
                    },
                }
            }
            EllipSource::Polyline(poly) => {
                //I did this fold because min requires the vertex to have the Ordering trait
                //but I forogot min_by exists taking a lambda, so I could compare them using
                //the value I need. However my first quick attempt wasn't working
                //Using min_by would probably be more effecietn than the fold
                //So this is probably worth coming back to...but it's a low priority
                //because the below code works.
                let x = poly
                    .vertices()
                    .fold(f64::MAX, |min_x, vtx| min_x.min(vtx.location.x));

                let max_x = poly
                    .vertices()
                    .fold(f64::MIN, |max_x, vtx| max_x.max(vtx.location.x));

                let y = poly
                    .vertices()
                    .fold(f64::MAX, |min_y, vtx| min_y.min(vtx.location.y));

                let max_y = poly
                    .vertices()
                    .fold(f64::MIN, |max_y, vtx| max_y.max(vtx.location.y));

                Ellipse {
                    x,
                    y: -max_y,
                    height: max_y - y,
                    width: max_x - x,
                    antialias: self.antialias,
                    style: StyleData {
                        line_weight: LineWeight::Thin,
                        ..self.style.unwrap_or_default()
                    },
                }
            }
            EllipSource::LwPolyline(lwpoly) => {
                let x = lwpoly
                    .vertices
                    .iter()
                    .fold(f64::MAX, |min_x, vtx| min_x.min(vtx.x));

                let max_x = lwpoly
                    .vertices
                    .iter()
                    .fold(f64::MIN, |max_x, vtx| max_x.max(vtx.x));

                let y = lwpoly
                    .vertices
                    .iter()
                    .fold(f64::MAX, |min_y, vtx| min_y.min(vtx.y));

                let max_y = lwpoly
                    .vertices
                    .iter()
                    .fold(f64::MIN, |max_y, vtx| max_y.max(vtx.y));

                Ellipse {
                    x,
                    y: -max_y,
                    height: max_y - y,
                    width: max_x - x,
                    antialias: self.antialias,
                    style: StyleData {
                        line_weight: LineWeight::Thin,
                        ..self.style.unwrap_or_default()
                    },
                }
            },
        }
    }
}


impl From<&Ellipse> for XMLElement {
    fn from(ell: &Ellipse) -> Self {
        let mut ell_xml = XMLElement::new("ellipse");
        ell_xml.add_attribute("x", two_dec(ell.x));
        ell_xml.add_attribute("y", two_dec(ell.y));
        ell_xml.add_attribute("width", two_dec(ell.width));
        ell_xml.add_attribute("height", two_dec(ell.height));
        ell_xml.add_attribute("antialias", ell.antialias);
        ell_xml.add_attribute("style", &ell.style);
        ell_xml
    }
}

impl Bounding for Ellipse {
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

impl ScaleEntity for Ellipse {
    fn scale(&mut self, fact_x: f64, fact_y: f64) {
        self.x *= fact_x;
        self.y *= fact_y;
        self.width *= fact_x;
        self.height *= fact_y;
    }
}
