use crate::qelmt::Bounding;
use crate::qelmt::style::LineWeight;
use crate::qelmt::style::StyleData;

use super::LineEnd;
use super::ScaleEntity;
use super::two_dec;
use dxf::entities::{self, LwPolyline, Polyline};
use simple_xml_builder::XMLElement;

#[derive(Debug)]
pub struct Line {
    length2: f64,
    end2: LineEnd,
    length1: f64,

    //need to brush up on my Rust scoping rules, isn't there a way to make this pub to just the module?
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,

    style: StyleData,
    end1: LineEnd,
    antialias: bool,
}

enum LineSource<'a> {
    Line(&'a entities::Line),
    PolyLine(&'a Polyline),
    LwPolyLine(&'a LwPolyline),
}

pub struct LineBuilder<'a> {
    source: LineSource<'a>,
    style: Option<StyleData>,
    antialias: bool,
}

impl<'a> LineBuilder<'a> {
    pub fn from_line(line: &'a entities::Line) -> Self {
        Self {
            source: LineSource::Line(line),
            style: None,
            antialias: false,
        }
    }

    pub fn from_polyline(poly: &'a Polyline) -> Self {
        Self {
            source: LineSource::PolyLine(poly),
            style: None,
            antialias: false,
        }
    }

    pub fn from_lwpolyline(poly: &'a LwPolyline) -> Self {
        Self {
            source: LineSource::LwPolyLine(poly),
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
        Self { antialias, ..self }
    }

    pub fn build(self) -> Result<Line, &'static str/*TODO: Need Better Error*/> {
        Ok(match self.source {
            LineSource::Line(line) => {
                Line {
                    x1: line.p1.x,
                    y1: -line.p1.y,
                    length1: 1.5, //why is this statically set at 1.5?
                    end1: LineEnd::None,
                    x2: line.p2.x,
                    y2: -line.p2.y,
                    length2: 1.5, //why is this statically set at 1.5?
                    end2: LineEnd::None,
                    antialias: self.antialias,
                    style: if line.thickness > 0.5 {
                        self.style.unwrap_or_default()
                    } else {
                        StyleData {
                            line_weight: LineWeight::Thin,
                            ..self.style.unwrap_or_default()
                        }
                    },
                }
            }
            LineSource::PolyLine(poly) => {
                if poly.__vertices_and_handles.len() != 2 {
                    return Err("Error can't convert polyline with more than 2 points into a Line");
                }

                Line {
                    x1: poly.__vertices_and_handles[0].0.location.x,
                    y1: -poly.__vertices_and_handles[0].0.location.y,
                    length1: 1.5, //why is this statically set at 1.5?
                    end1: LineEnd::None,
                    x2: poly.__vertices_and_handles[1].0.location.x,
                    y2: -poly.__vertices_and_handles[1].0.location.y,
                    length2: 1.5, //why is this statically set at 1.5?
                    end2: LineEnd::None,
                    antialias: self.antialias,
                    style: if poly.thickness > 0.5 {
                        self.style.unwrap_or_default()
                    } else {
                        StyleData {
                            line_weight: LineWeight::Thin,
                            ..self.style.unwrap_or_default()
                        }
                    },
                }
            }
            LineSource::LwPolyLine(lw_poly) => {
                if lw_poly.vertices.len() != 2 {
                    return Err("Error can't convert polyline with more than 2 points into a Line");
                }

                Line {
                    x1: lw_poly.vertices[0].x,
                    y1: -lw_poly.vertices[0].y,
                    length1: 1.5, //why is this statically set at 1.5?
                    end1: LineEnd::None,
                    x2: lw_poly.vertices[1].x,
                    y2: -lw_poly.vertices[1].y,
                    length2: 1.5, //why is this statically set at 1.5?
                    end2: LineEnd::None,
                    antialias: self.antialias,
                    style: if lw_poly.thickness > 0.1 {
                        self.style.unwrap_or_default()
                    } else {
                        StyleData {
                            line_weight: LineWeight::Thin,
                            ..self.style.unwrap_or_default()
                        }
                    },
                }
            }
        })
    }
}

pub struct Leader(pub Vec<Line>);

pub struct LeaderBuilder<'a> {
    leader: &'a entities::Leader,
    style: Option<StyleData>,
    antialias: bool
}

impl<'a> LeaderBuilder<'a> {
    pub fn new(leader: &'a entities::Leader) -> Self {
        Self {
            leader,
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

    pub fn build(self) -> Leader {
        Leader(
            self.leader
                .vertices
                .windows(2)
                .enumerate()
                .map(|(cnt, pt_slice)| {
                    let end1 = if self.leader.use_arrowheads && cnt == 0 {
                        LineEnd::SimpleArrow
                    } else {
                        LineEnd::None
                    };

                    Line {
                        x1: pt_slice[0].x,
                        y1: -pt_slice[0].y,
                        length1: 1.5, //In order to get the arrow sizing, I need to read in the dimension styling first
                        end1,
                        x2: pt_slice[1].x,
                        y2: -pt_slice[1].y,
                        length2: 1.5, //In order to get the arrow sizing, I need to read in the dimension styling first
                        end2: LineEnd::None,

                        antialias: self.antialias,
                        
                        //Leader style information seems to come from dimension style info
                        //for now I'll either just use the default or whats passed in.
                        //style: self.style.unwrap_or_default(),
                        style: StyleData::default(),

                    }
                })
                .collect(),
        )
    }
}

impl From<&Line> for XMLElement {
    fn from(line: &Line) -> Self {
        let mut line_xml = XMLElement::new("line");
        line_xml.add_attribute("x1", two_dec(line.x1));
        line_xml.add_attribute("y1", two_dec(line.y1));
        line_xml.add_attribute("length1", two_dec(line.length1));
        line_xml.add_attribute("end1", &line.end1);
        line_xml.add_attribute("x2", two_dec(line.x2));
        line_xml.add_attribute("y2", two_dec(line.y2));
        line_xml.add_attribute("length2", two_dec(line.length2));
        line_xml.add_attribute("end2", &line.end2);
        line_xml.add_attribute("antialias", line.antialias);
        line_xml.add_attribute("style", &line.style);
        line_xml
    }
}

impl Bounding for Line {
    fn left_bound(&self) -> f64 {
        self.x1.min(self.x2)
    }

    fn right_bound(&self) -> f64 {
        self.x1.max(self.x2)
    }

    fn top_bound(&self) -> f64 {
        self.y1.min(self.y2)
    }

    fn bot_bound(&self) -> f64 {
        self.y1.max(self.y2)
    }
}

impl ScaleEntity for Line {
    fn scale(&mut self, fact_x: f64, fact_y: f64) {
        self.x1 *= fact_x;
        self.x2 *= fact_x;

        self.y1 *= fact_y;
        self.y2 *= fact_y;

        //while writing this scaling code, I'm looking at
        //QET_ElementScaler from plc-user to see if there are
        //any easy to overlook mistakes that I might make
        //doing the scaling. It seems they limit these lengths
        //to 99.0, but I'm not sure why at the moment. I'll go
        //ahead and limit them as well, and try to come back to
        //figure out what the purpose here is
        self.length1 *= fact_x.min(fact_y);
        self.length1 = self.length1.min(99.0);

        self.length2 *= fact_x.min(fact_y);
        self.length2 = self.length2.min(99.0);
    }
}
