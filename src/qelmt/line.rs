use super::two_dec;
use super::LineEnd;
use super::ScaleEntity;
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

    style: String,
    end1: LineEnd,
    antialias: bool,
}

pub struct Leader(pub Vec<Line>);

impl From<&entities::Line> for Line {
    fn from(line: &entities::Line) -> Self {
        Line {
            x1: line.p1.x,
            y1: -line.p1.y,
            length1: 1.5, //why is this statically set at 1.5?
            end1: LineEnd::None,
            x2: line.p2.x,
            y2: -line.p2.y,
            length2: 1.5, //why is this statically set at 1.5?
            end2: LineEnd::None,

            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: if line.thickness > 0.5 {
                "line-style:normal;line-weight:normal;filling:none;color:black"
            } else {
                "line-style:normal;line-weight:thin;filling:none;color:black"
            }
            .into(),
        }
    }
}

impl TryFrom<&Polyline> for Line {
    type Error = &'static str; //add better error later

    fn try_from(poly: &Polyline) -> Result<Self, Self::Error> {
        if poly.__vertices_and_handles.len() != 2 {
            return Err("Error can't convert polyline with more than 2 points into a Line");
        }

        Ok(Line {
            x1: poly.__vertices_and_handles[0].0.location.x,
            y1: -poly.__vertices_and_handles[0].0.location.y,
            length1: 1.5, //why is this statically set at 1.5?
            end1: LineEnd::None,
            x2: poly.__vertices_and_handles[1].0.location.x,
            y2: -poly.__vertices_and_handles[1].0.location.y,
            length2: 1.5, //why is this statically set at 1.5?
            end2: LineEnd::None,

            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: if poly.thickness > 0.5 {
                "line-style:normal;line-weight:normal;filling:none;color:black"
            } else {
                "line-style:normal;line-weight:thin;filling:none;color:black"
            }
            .into(),
        })
    }
}

impl TryFrom<&LwPolyline> for Line {
    type Error = &'static str; //add better error later

    fn try_from(poly: &LwPolyline) -> Result<Self, Self::Error> {
        if poly.vertices.len() != 2 {
            return Err("Error can't convert polyline with more than 2 points into a Line");
        }

        Ok(Line {
            x1: poly.vertices[0].x,
            y1: -poly.vertices[0].y,
            length1: 1.5, //why is this statically set at 1.5?
            end1: LineEnd::None,
            x2: poly.vertices[1].x,
            y2: -poly.vertices[1].y,
            length2: 1.5, //why is this statically set at 1.5?
            end2: LineEnd::None,

            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: if poly.thickness > 0.1 {
                "line-style:normal;line-weight:normal;filling:none;color:black"
            } else {
                "line-style:normal;line-weight:thin;filling:none;color:black"
            }
            .into(),
        })
    }
}

impl From<&entities::Leader> for Leader {
    fn from(leader: &entities::Leader) -> Self {
        Leader(
            leader
                .vertices
                .windows(2)
                .enumerate()
                .map(|(cnt, pt_slice)| {
                    let end1 = if leader.use_arrowheads && cnt == 0 {
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

                        //in the original code antialias is always set to false...I'm guessing for performance
                        //reasons...I'm trying to think if there is a time we might want to turn it on?
                        antialias: false,
                        //looks like line thickenss and color information I *might* need to grab from a dimension style
                        //entity which I haven't implemented yet
                        /*style: if line.thickness > 0.5 {
                            "line-style:normal;line-weight:normal;filling:none;color:black"
                        } else {
                            "line-style:normal;line-weight:thin;filling:none;color:black"
                        }
                        .into(),*/
                        style: "line-style:normal;line-weight:normal;filling:none;color:black"
                            .into(),
                    }
                })
                .collect(),
        )
    }
}

impl From<&Line> for XMLElement {
    fn from(line: &Line) -> Self {
        let mut line_xml: XMLElement = XMLElement::new("line");
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

impl ScaleEntity for Line {
    fn scale(&mut self, fact: f64) {
        self.x1 *= fact;
        self.x2 *= fact;

        self.y1 *= fact;
        self.y2 *= fact;

        //while writing this scaling code, I'm looking at
        //QET_ElementScaler from plc-user to see if there are
        //any easy to overlook mistakes that I might make
        //doing the scaling. It seems they limit these lengths
        //to 99.0, but I'm not sure why at the moment. I'll go
        //ahead and limit them as well, and try to come back to
        //figure out what the purpose here is
        self.length1 *= fact;
        self.length1 = self.length1.min(99.0);

        self.length2 *= fact;
        self.length2 = self.length2.min(99.0);
    }

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
