use dxf::entities::{AttributeDefinition, Entity, EntityType};
use dxf::entities::{LwPolyline, Polyline};
use dxf::enums::{AttachmentPoint, HorizontalTextJustification, Units, VerticalTextJustification};
use dxf::{Block, Drawing};
use dynamictext::DTextBuilder;
use hex_color::HexColor;
use itertools::Itertools;
use simple_xml_builder::XMLElement;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::f64::consts::PI;
use std::fmt::Display;
use uuid::Uuid;

use tracing::{error, info, span, trace, Level};

pub mod arc;
pub use arc::Arc;

pub mod line;
pub use line::{Leader, Line};

pub mod text;
pub use text::Text;

pub mod dynamictext;
pub use dynamictext::DynamicText;

pub mod polygon;
pub use polygon::Polygon;

pub mod ellipse;
pub use ellipse::Ellipse;

pub mod rectangle;
pub use rectangle::Rectangle;

pub mod style;
//pub use style::StyleData;

fn find_block<'a>(drw: &'a Drawing, name: &str) -> Option<&'a Block> {
    //this is ugly there has to be a cleaner way to filter this....but for my first attempt at pulling the
    //blocks out of the drawing it works.
    //I mean would this ever return more than 1? I would assume block names have to be unique?
    //but maybe not, the blocks have a handle, which is a u64. There is a get by handle function
    //but not a get by name function....maybe the handle is what is unique and there can be duplicate names?
    //a quick glance through the dxf code it looks like the handle might be given to the library user when inserting
    //an entity? So I don't think there is any easy way to get the handle
    drw.blocks().filter(|bl| bl.name == name).take(1).next()
}

//Due to the way I'm using this Either enum, clippy apparenlty can't detect that it's used
//so I get an unused code warning. Apparently the solution is either alow unused for the enum
//or restructure my code to make it more clear to clippy that it is indead used...
#[derive(Debug)]
#[allow(unused)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Debug)]
pub struct Definition {
    r#type: ItemType,
    width: i64,
    height: i64,
    hotspot_x: i64,
    hotspot_y: i64,
    version: String,
    link_type: LinkType,
    uuid: ElmtUuid,
    names: Names,
    element_infos: Option<ElemInfos>,
    informations: &'static str,
    description: Description,
    //counts
}

pub trait Mean {
    fn mean(self) -> f64;
}

impl<F, T> Mean for T
where
    T: Iterator<Item = F>,
    F: std::borrow::Borrow<f64>,
{
    fn mean(self) -> f64 {
        self.zip(1..).fold(0., |s, (e, i)| {
            (*e.borrow() + s * f64::from(i - 1)) / f64::from(i)
        })
    }
}

trait Bounding {
    fn left_bound(&self) -> f64;
    fn right_bound(&self) -> f64;

    fn top_bound(&self) -> f64;
    fn bot_bound(&self) -> f64;

    fn bounding_area(&self) -> f64 {
        ((self.right_bound() - self.left_bound()) * (self.bot_bound() - self.top_bound())).abs()
    }
}

trait ScaleEntity: Bounding {
    fn scale(&mut self, fact_x: f64, fact_y: f64);
}

trait Arity {
    fn match_range() -> std::ops::RangeInclusive<f64> {
        //this boundary of 2% has been chosen arbitrarily, I might adjust this later
        //I know in one of my sample files, I'm getting a value of 0.99....
        //since Associated Constants in a trait can't have a default value
        //I'm using this function that defaults to a constant range of 2%
        //Then I could also easily overwrite it if I wanted to change the tolerance
        //for a specific type
        0.98..=1.02
    }

    // Uses the Shoelace Formula to calculate the area of a polygon from an
    // iterator of coordinates
    fn area<'a>(coords: impl Clone + ExactSizeIterator<Item = &'a dxf::Point>) -> f64 {
        let len = coords.len();
        (coords
            .circular_tuple_windows()
            .map(|(v1, v2)| (v1.x * v2.y) - (v1.y * v2.x))
            .take(len)
            .sum::<f64>()
            / 2.0)
            .abs()
    }
}

trait Circularity: Arity {
    fn is_circular(&self) -> bool;
}

trait Rectangularity: Arity {
    fn is_rectangular(&self) -> bool;
}

impl Bounding for Polyline {
    fn left_bound(&self) -> f64 {
        if let Some(vtx) = self.vertices().min_by(|v1, v2| {
            v1.location
                .x
                .partial_cmp(&v2.location.x)
                .unwrap_or(Ordering::Greater)
        }) {
            vtx.location.x
        } else {
            0.0
        }
    }

    fn right_bound(&self) -> f64 {
        if let Some(vtx) = self.vertices().max_by(|v1, v2| {
            v1.location
                .x
                .partial_cmp(&v2.location.x)
                .unwrap_or(Ordering::Less)
        }) {
            vtx.location.x
        } else {
            0.0
        }
    }

    fn top_bound(&self) -> f64 {
        if let Some(vtx) = self.vertices().max_by(|v1, v2| {
            v1.location
                .y
                .partial_cmp(&v2.location.y)
                .unwrap_or(Ordering::Less)
        }) {
            vtx.location.y
        } else {
            0.0
        }
    }

    fn bot_bound(&self) -> f64 {
        if let Some(vtx) = self.vertices().min_by(|v1, v2| {
            v1.location
                .y
                .partial_cmp(&v2.location.y)
                .unwrap_or(Ordering::Less)
        }) {
            vtx.location.y
        } else {
            0.0
        }
    }
}

impl Arity for Polyline {}

impl Circularity for Polyline {
    fn is_circular(&self) -> bool {
        let perim: f64 = {
            let tmp_pts: Vec<dxf::Point> = self.vertices().map(|v| v.clone().location).collect();
            let len = tmp_pts.len();
            tmp_pts
                .into_iter()
                .circular_tuple_windows()
                .map(|(fst, sec)| ((fst.x - sec.x).powf(2.0) + (fst.y - sec.y).powf(2.0)).sqrt())
                .take(len)
                .sum()
        };

        let vertices: Vec<_> = self.vertices().map(|v| &v.location).collect();
        let area = Polyline::area(vertices.into_iter());
        let t_ratio = 4.0 * PI * area / perim.powf(2.0);

        Self::match_range().contains(&t_ratio)
    }
}

impl Rectangularity for Polyline {
    fn is_rectangular(&self) -> bool {
        let vertices: Vec<_> = self.vertices().map(|v| &v.location).collect();
        let bounding_area = self.bounding_area();

        let area = Polyline::area(vertices.into_iter());
        Self::match_range().contains(&(area / bounding_area))
    }
}

impl Bounding for LwPolyline {
    fn left_bound(&self) -> f64 {
        if let Some(vtx) = self
            .vertices
            .iter()
            .min_by(|v1, v2| v1.x.partial_cmp(&v2.x).unwrap_or(Ordering::Greater))
        {
            vtx.x
        } else {
            0.0
        }
    }

    fn right_bound(&self) -> f64 {
        if let Some(vtx) = self
            .vertices
            .iter()
            .max_by(|v1, v2| v1.x.partial_cmp(&v2.x).unwrap_or(Ordering::Less))
        {
            vtx.x
        } else {
            0.0
        }
    }

    fn top_bound(&self) -> f64 {
        if let Some(vtx) = self
            .vertices
            .iter()
            .max_by(|v1, v2| v1.y.partial_cmp(&v2.y).unwrap_or(Ordering::Less))
        {
            vtx.y
        } else {
            0.0
        }
    }

    fn bot_bound(&self) -> f64 {
        if let Some(vtx) = self
            .vertices
            .iter()
            .min_by(|v1, v2| v1.y.partial_cmp(&v2.y).unwrap_or(Ordering::Less))
        {
            vtx.y
        } else {
            0.0
        }
    }
}

impl Arity for LwPolyline {}

impl Circularity for LwPolyline {
    fn is_circular(&self) -> bool {
        let perim: f64 = self
            .vertices
            .iter()
            .circular_tuple_windows()
            .map(|(fst, sec)| {
                ((fst.x - sec.x).powf(2.0) + (fst.y - sec.y).powf(2.0))
                    .abs()
                    .sqrt()
            })
            .take(self.vertices.len())
            .sum();

        let vertices: Vec<_> = self
            .vertices
            .iter()
            .map(|v| dxf::Point::new(v.x, v.y, 0.0))
            .collect();

        let area = LwPolyline::area(vertices.iter());
        let t_ratio = 4.0 * PI * area / perim.powf(2.0);

        Self::match_range().contains(&t_ratio)
    }
}

impl Rectangularity for LwPolyline {
    fn is_rectangular(&self) -> bool {
        let bounding_area = self.bounding_area();
        let vertices: Vec<_> = self
            .vertices
            .iter()
            .map(|v| dxf::Point::new(v.x, v.y, 0.0))
            .collect();
        let area = LwPolyline::area(vertices.iter());

        Self::match_range().contains(&(area / bounding_area))
    }
}

impl Definition {
    pub fn new(name: impl Into<String>, spline_step: Option<f64>, drw: &Drawing) -> Self {
        /*for st in drw.styles() {
            dbg!(st);
        }*/
        let scale_factor = Self::scale_factor(drw.header.default_drawing_units);
        let description = {
            let mut description: Description = (drw, spline_step).into();
            description.scale(scale_factor, scale_factor);
            description
        };

        //The below calculation for width and hotspot_x are taken from the qet source code
        let (width, hotspot_x) = {
            let tmp_width = description.right_bound() - description.left_bound();
            let int_width = tmp_width.round() as i64;
            let upwidth = ((int_width / 10) * 10) + 10;
            let xmargin = (upwidth as f64 - tmp_width).round();

            let width = if int_width % 10 > 6 {
                upwidth + 10
            } else {
                upwidth
            };

            (
                width,
                -((description.left_bound() - (xmargin / 2.0)).round() as i64),
            )
        };

        //The below calculation for height and hotspot_y are taken from the qet source code
        let (height, hotspot_y) = {
            let tmp_height = description.bot_bound() - description.top_bound();
            let int_height = tmp_height.round() as i64;
            let upheight = ((int_height / 10) * 10) + 10;
            let ymargin = (upheight as f64 - tmp_height).round();

            let height = if int_height % 10 > 6 {
                upheight + 10
            } else {
                upheight
            };

            (
                height,
                -((description.top_bound() - (ymargin / 2.0)).round() as i64),
            )
        };

        Definition {
            r#type: ItemType::Element,
            width,
            height,
            hotspot_x,
            hotspot_y,
            version: "0.8.0".into(),
            link_type: LinkType::Simple,
            uuid: Uuid::new_v4().into(),
            names: Names {
                names: vec![Name {
                    lang: "en".into(),
                    value: name.into(), //need to truncate the extension
                }],
            },
            element_infos: None,
            informations: "Created using dxf2elmt!",
            description,
        }
    }

    fn scale_factor(unit: Units) -> f64 {
        //so per discussion at https://qelectrotech.org/forum/viewtopic.php?pid=20685#p20685
        //we are in agreement to scale things to 1mm = 2px;
        //all the below values are the converted equivalent of 2px per 1mm in the designated unit
        //unit conversions taken from: https://www.unitconverters.net/length-converter.html
        match unit {
            Units::Unitless => 1.0, //for now if the drawing is untiless don't scale it
            Units::Inches => 50.8,
            Units::Feet => 609.6,
            Units::Miles | Units::USSurveyMile => 3_218_694.437_4,
            Units::Millimeters => 2.0,
            Units::Centimeters => 20.0,
            Units::Meters => 2_000.0,
            Units::Kilometers => 2_000_000.0,
            Units::Microinches => 50.8E-6,
            Units::Mils => 0.0508,
            Units::Yards => 1_828.8,
            Units::Angstroms => 2.0E-7,
            Units::Nanometers => 2.0e-6,
            Units::Microns => 0.002,
            Units::Decimeters => 200.0,
            Units::Decameters => 20_000.0,
            Units::Hectometers => 200_000.0,
            Units::Gigameters => 2.0e12,
            Units::AstronomicalUnits => 299_195_741_382_000.0,
            Units::LightYears => 18_921_460_945_160_086_000.0,
            Units::Parsecs => 61_713_551_625_599_170_000.0,
            Units::USSurveyFeet => 609.601_219_2,
            Units::USSurveyInch => 50.800_101_6,

            //I'm finding very little references to US Survey yard at all. The only real
            //link I could find was on the Wikipedia page for the Yard, which stated:
            //"The US survey yard is very slightly longer." and linked to the US Survey Foot page
            //I'll assume for now that 1 US Survey Yard is equal to 3 US Survey Feet. Which seems
            //like a reasonable assumption, and would result in something slightly larger than a yard
            Units::USSurveyYard => 1_828.803_657_6,
        }
    }
}

impl From<&Definition> for XMLElement {
    fn from(def: &Definition) -> Self {
        let mut def_xml = XMLElement::new("definition");
        def_xml.add_attribute("height", def.height);
        def_xml.add_attribute("width", def.width);
        def_xml.add_attribute("hotspot_x", def.hotspot_x);
        def_xml.add_attribute("hotspot_y", def.hotspot_y);
        def_xml.add_attribute("version", &def.version);
        def_xml.add_attribute("link_type", &def.link_type);
        def_xml.add_attribute("type", &def.r#type);

        def_xml.add_child((&def.uuid).into());
        def_xml.add_child((&def.names).into());
        if let Some(einfos) = &def.element_infos {
            def_xml.add_child(einfos.into());
        }

        let mut info_elmt = XMLElement::new("informations");
        info_elmt.add_text(def.informations);
        def_xml.add_child(info_elmt);

        def_xml.add_child((&def.description).into());

        def_xml
    }
}

#[derive(Debug)]
pub(crate) enum Objects {
    Arc(Arc),
    Ellipse(Ellipse),
    Polygon(Polygon),
    Rectangle(Rectangle),
    DynamicText(DynamicText),
    Text(Text),
    Line(Line),
    //Terminal(Terminal),
    Group(Vec<Objects>),
}

impl Objects {
    pub fn descendants(&self) -> Descendants<'_> {
        Descendants {
            stack: vec![self.children()],
        }
    }

    pub fn children(&self) -> Children<'_> {
        match self {
            Objects::Group(l) => Children { slice: l.iter() },
            _ => Children { slice: [].iter() },
        }
    }
}

pub(crate) struct Descendants<'a> {
    stack: Vec<Children<'a>>,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = &'a Objects;
    fn next(&mut self) -> Option<Self::Item> {
        //let iter_span = span!(Level::TRACE, "Iterating Object Descendants");
        //let _span_guard = iter_span.enter();
        while let Some(last) = self.stack.last_mut() {
            if let Some(obj) = last.next() {
                //trace!("Found more children");
                self.stack.push(obj.children());
                return Some(obj);
            }

            self.stack.pop();
        }
        None
    }
}

pub(crate) struct Children<'a> {
    slice: std::slice::Iter<'a, Objects>,
}

impl<'a> Iterator for Children<'a> {
    type Item = &'a Objects;
    fn next(&mut self) -> Option<Self::Item> {
        self.slice.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.slice.len(), None)
    }
}

impl Bounding for Objects {
    fn left_bound(&self) -> f64 {
        match self {
            Objects::Arc(arc) => arc.left_bound(),
            Objects::Ellipse(ellipse) => ellipse.left_bound(),
            Objects::Polygon(polygon) => polygon.left_bound(),
            Objects::Rectangle(rectangle) => rectangle.left_bound(),
            Objects::DynamicText(dynamic_text) => dynamic_text.left_bound(),
            Objects::Text(text) => text.left_bound(),
            Objects::Line(line) => line.left_bound(),
            Objects::Group(vec) => {
                let lb = vec.iter().min_by(|ob1, ob2| {
                    ob1.left_bound()
                        .partial_cmp(&ob2.left_bound())
                        .unwrap_or(Ordering::Greater)
                });

                if let Some(lb) = lb {
                    lb.left_bound()
                } else {
                    0.0
                }
            }
        }
    }

    fn right_bound(&self) -> f64 {
        match self {
            Objects::Arc(arc) => arc.right_bound(),
            Objects::Ellipse(ellipse) => ellipse.right_bound(),
            Objects::Polygon(polygon) => polygon.right_bound(),
            Objects::Rectangle(rectangle) => rectangle.right_bound(),
            Objects::DynamicText(dynamic_text) => dynamic_text.right_bound(),
            Objects::Text(text) => text.right_bound(),
            Objects::Line(line) => line.right_bound(),
            Objects::Group(vec) => {
                let rb = vec.iter().max_by(|ob1, ob2| {
                    ob1.right_bound()
                        .partial_cmp(&ob2.right_bound())
                        .unwrap_or(Ordering::Less)
                });

                if let Some(rb) = rb {
                    rb.right_bound()
                } else {
                    0.0
                }
            }
        }
    }

    fn top_bound(&self) -> f64 {
        match self {
            Objects::Arc(arc) => arc.top_bound(),
            Objects::Ellipse(ellipse) => ellipse.top_bound(),
            Objects::Polygon(polygon) => polygon.top_bound(),
            Objects::Rectangle(rectangle) => rectangle.top_bound(),
            Objects::DynamicText(dynamic_text) => dynamic_text.top_bound(),
            Objects::Text(text) => text.top_bound(),
            Objects::Line(line) => line.top_bound(),
            Objects::Group(vec) => {
                let tb = vec.iter().min_by(|ob1, ob2| {
                    ob1.top_bound()
                        .partial_cmp(&ob2.top_bound())
                        .unwrap_or(Ordering::Greater)
                });

                if let Some(tb) = tb {
                    tb.top_bound()
                } else {
                    0.0
                }
            }
        }
    }

    fn bot_bound(&self) -> f64 {
        match self {
            Objects::Arc(arc) => arc.bot_bound(),
            Objects::Ellipse(ellipse) => ellipse.bot_bound(),
            Objects::Polygon(polygon) => polygon.bot_bound(),
            Objects::Rectangle(rectangle) => rectangle.bot_bound(),
            Objects::DynamicText(dynamic_text) => dynamic_text.bot_bound(),
            Objects::Text(text) => text.bot_bound(),
            Objects::Line(line) => line.bot_bound(),
            Objects::Group(vec) => {
                let bb = vec.iter().max_by(|ob1, ob2| {
                    ob1.bot_bound()
                        .partial_cmp(&ob2.bot_bound())
                        .unwrap_or(Ordering::Less)
                });

                if let Some(bb) = bb {
                    bb.bot_bound()
                } else {
                    0.0
                }
            }
        }
    }
}

impl ScaleEntity for Objects {
    fn scale(&mut self, fact_x: f64, fact_y: f64) {
        match self {
            Objects::Arc(arc) => arc.scale(fact_x, fact_y),
            Objects::Ellipse(ellipse) => ellipse.scale(fact_x, fact_y),
            Objects::Polygon(polygon) => polygon.scale(fact_x, fact_y),
            Objects::Rectangle(rectangle) => rectangle.scale(fact_x, fact_y),
            Objects::DynamicText(dynamic_text) => dynamic_text.scale(fact_x, fact_y),
            Objects::Text(text) => text.scale(fact_x, fact_y),
            Objects::Line(line) => line.scale(fact_x, fact_y),
            Objects::Group(vec) => vec.iter_mut().for_each(|ob| ob.scale(fact_x, fact_y)),
        }
    }
}

#[derive(Debug)]
struct ScaleFactor {
    x: f64,
    y: f64,
}

impl Default for ScaleFactor {
    fn default() -> Self {
        ScaleFactor { x: 1.0, y: 1.0 }
    }
}

#[derive(Default, Debug)]
pub(crate) struct Offset {
    x: f64,
    y: f64,
}

#[derive(Debug)]
pub struct ObjectsBuilder<'a> {
    ent: &'a Entity,
    spline_step: Option<f64>,
    blocks: &'a [&'a Block],
    offset: Offset,
    scale_fact: ScaleFactor,
}

impl<'a> ObjectsBuilder<'a> {
    pub fn new(ent: &'a Entity, spline_step: Option<f64>) -> Self {
        Self {
            ent,
            spline_step,
            blocks: &[],
            offset: Offset::default(),
            scale_fact: ScaleFactor::default(),
        }
    }

    pub fn blocks(self, blocks: &'a [&'a Block]) -> Self {
        Self { blocks, ..self }
    }

    pub fn offsets(self, x: f64, y: f64) -> Self {
        Self {
            offset: Offset { x, y },
            ..self
        }
    }

    pub fn scaling(self, fact_x: f64, fact_y: f64) -> Self {
        Self {
            scale_fact: ScaleFactor {
                x: fact_x,
                y: fact_y,
            },
            ..self
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn build(self) -> Result<Objects, &'static str /*add better error later*/> {
        match &self.ent.specific {
            EntityType::Circle(circle) => {
                let mut ellipse: Ellipse = circle.into();

                ellipse.scale(self.scale_fact.x, self.scale_fact.y);
                ellipse.x += self.offset.x;
                ellipse.y -= self.offset.y;
                Ok(Objects::Ellipse(ellipse))
            }
            EntityType::Line(line) => {
                let mut line: Line = line.into();

                line.scale(self.scale_fact.x, self.scale_fact.y);

                line.x1 += self.offset.x;
                line.y1 -= self.offset.y;

                line.x2 += self.offset.x;
                line.y2 -= self.offset.y;

                Ok(Objects::Line(line))
            }
            EntityType::Arc(arc) => {
                let mut arc: Arc = arc.into();

                arc.scale(self.scale_fact.x, self.scale_fact.y);

                arc.x += self.offset.x;
                arc.y -= self.offset.y;

                Ok(Objects::Arc(arc))
            }
            EntityType::Spline(spline) => {
                let mut poly: Polygon = (spline, self.spline_step).into();

                match poly.coordinates.len() {
                    0 | 1 => Err("Error removing empty Spline"),
                    //I'll need to improve my understanding of splines and the math here
                    //to make sure I do this correctly.
                    //2 => //convert to line
                    _ => {
                        poly.scale(self.scale_fact.x, self.scale_fact.y);
                        for cord in &mut poly.coordinates {
                            cord.x += self.offset.x;
                            cord.y -= self.offset.y;
                        }

                        Ok(Objects::Polygon(poly))
                    }
                }
            }
            EntityType::Text(text) => {
                Ok(
                    //right now the dxf2elmt defaults to making all text Static Text...
                    //it was requested by the QET devs to add in support for Dynamic text
                    //which was added, but it defaults to OFF, and QET doesn't pass the parameter
                    //to enable it...I'm wondering if it makes more sense to default to use dynamic text
                    //for now I'll set it to use dynamic text, and once I get the CLI flag passing through
                    //I might change the default parameter to use Dynamic Text
                    if false {
                        //how best to pass in the flag for dynamic text or not....should the flag also default to true?
                        let mut text: Text = (
                            text,
                            HexColor::from_u32(self.ent.common.color_24_bit as u32),
                        )
                            .into();

                        text.scale(self.scale_fact.x, self.scale_fact.y);

                        text.x += self.offset.x;
                        text.y -= self.offset.y;

                        Objects::Text(text)
                    } else {
                        let mut dtext = DTextBuilder::from_text(text)
                            .color(HexColor::from_u32(self.ent.common.color_24_bit as u32))
                            .build();

                        dtext.scale(self.scale_fact.x, self.scale_fact.y);

                        dtext.x += self.offset.x;
                        dtext.y -= self.offset.y;

                        Objects::DynamicText(dtext)
                    },
                )
            }
            EntityType::Ellipse(ellipse) => {
                let mut ellipse: Ellipse = ellipse.into();

                ellipse.scale(self.scale_fact.x, self.scale_fact.y);
                ellipse.x += self.offset.x;
                ellipse.y -= self.offset.y;

                Ok(Objects::Ellipse(ellipse))
            }
            EntityType::MText(mtext) => {
                Ok(
                    //right now the dxf2elmt defaults to making all text Static Text...
                    //it was requested by the QET devs to add in support for Dynamic text
                    //which was added, but it defaults to OFF, and QET doesn't pass the parameter
                    //to enable it...I'm wondering if it makes more sense to default to use dynamic text
                    //for now I'll set it to use dynamic text, and once I get the CLI flag passing through
                    //I might change the default parameter to use Dynamic Text
                    if false {
                        //how best to pass in the flag for dynamic text or not....should the flag also default to true?
                        /*let mut text: Text =
                            (mtext, HexColor::from_u32(ent.common.color_24_bit as u32)).into();
                            text.x += offset_x;
                            text.y -= offset_y;
                        Objects::Text(text)*/
                        todo!();
                    } else {
                        let mut dtext = DTextBuilder::from_mtext(mtext)
                            .color(HexColor::from_u32(self.ent.common.color_24_bit as u32))
                            .build();

                        dtext.scale(self.scale_fact.x, self.scale_fact.y);

                        dtext.x += self.offset.x;
                        dtext.y -= self.offset.y;

                        Objects::DynamicText(dtext)
                    },
                )
            }
            EntityType::Polyline(polyline) => match polyline.__vertices_and_handles.len() {
                0 | 1 => Err("Error empty Polyline"),
                2 => {
                    let mut line = Line::try_from(polyline)?;

                    line.scale(self.scale_fact.x, self.scale_fact.x);

                    line.x1 += self.offset.x;
                    line.y1 -= self.offset.y;

                    line.x2 += self.offset.x;
                    line.y2 -= self.offset.y;

                    Ok(Objects::Line(line))
                }
                _ => {
                    if let Ok(mut ellipse) = Ellipse::try_from(polyline) {
                        ellipse.scale(self.scale_fact.x, self.scale_fact.y);

                        ellipse.x += self.offset.x;
                        ellipse.y -= self.offset.y;

                        Ok(Objects::Ellipse(ellipse))
                    } else if let Ok(mut rectangle) = Rectangle::try_from(polyline) {
                        rectangle.scale(self.scale_fact.x, self.scale_fact.y);

                        rectangle.x += self.offset.x;
                        rectangle.y -= self.offset.y;

                        Ok(Objects::Rectangle(rectangle))
                    } else {
                        let mut poly: Polygon = polyline.into();

                        poly.scale(self.scale_fact.x, self.scale_fact.y);

                        for cord in &mut poly.coordinates {
                            cord.x += self.offset.x;
                            cord.y -= self.offset.y;
                        }

                        Ok(Objects::Polygon(poly))
                    }
                }
            },
            EntityType::LwPolyline(lwpolyline) => match lwpolyline.vertices.len() {
                0 | 1 => Err("Error empty LwPolyline"),
                2 => {
                    let mut line = Line::try_from(lwpolyline)?;

                    line.scale(self.scale_fact.x, self.scale_fact.y);

                    line.x1 += self.offset.x;
                    line.y1 -= self.offset.y;

                    line.x2 += self.offset.x;
                    line.y2 -= self.offset.y;

                    Ok(Objects::Line(line))
                }
                _ => {
                    if let Ok(mut ellipse) = Ellipse::try_from(lwpolyline) {
                        ellipse.scale(self.scale_fact.x, self.scale_fact.y);

                        ellipse.x += self.offset.x;
                        ellipse.y -= self.offset.y;

                        Ok(Objects::Ellipse(ellipse))
                    } else if let Ok(mut rectangle) = Rectangle::try_from(lwpolyline) {
                        // Hmm there still seem to be some issues here.
                        // I have a few examples with a polyline that is a perfect
                        // rectangle, but it's not a "closed" polyline, and has 5 points.
                        // it's getting converted as a 5 point polygon, not a rectangle.
                        // need to dig into why...is the extra point instead of being a closed
                        // polygon throwing off the area calculation? That would be my guess
                        rectangle.scale(self.scale_fact.x, self.scale_fact.y);

                        rectangle.x += self.offset.x;
                        rectangle.y -= self.offset.y;

                        Ok(Objects::Rectangle(rectangle))
                    } else {
                        let mut poly: Polygon = lwpolyline.into();

                        poly.scale(self.scale_fact.x, self.scale_fact.y);

                        for cord in &mut poly.coordinates {
                            cord.x += self.offset.x;
                            cord.y -= self.offset.y;
                        }

                        Ok(Objects::Polygon(poly))
                    }
                }
            },
            EntityType::Solid(solid) => {
                let mut poly: Polygon = solid.into();

                poly.scale(self.scale_fact.x, self.scale_fact.y);

                for cord in &mut poly.coordinates {
                    cord.x += self.offset.x;
                    cord.y -= self.offset.y;
                }

                Ok(Objects::Polygon(poly))
            }
            EntityType::Insert(ins) => {
                //info!("Found an Insert Block: {ins:?}");
                info!("Found an Insert Block: {}", &ins.name);
                let Some(block) = self.blocks.iter().find(|bl| bl.name == ins.name) else {
                    error!("Block {} not found", ins.name);
                    return Err("Block Not Found");
                };
                trace!(
                    "Base Point: x: {} / y: {}",
                    block.base_point.x,
                    block.base_point.y
                );

                trace!("Creating Group from block {}. Pos(x:{}, y:{}). Offset(x:{}, y:{}). Scale(x:{}, y:{})",
                    ins.name, ins.location.x, ins.location.y, self.offset.x, self.offset.y, self.scale_fact.x * ins.x_scale_factor,
                    self.scale_fact.y * ins.y_scale_factor);
                Ok(Objects::Group(
                    block
                        .entities
                        .iter()
                        .filter_map(|ent| {
                            ObjectsBuilder::new(ent, self.spline_step)
                                .offsets(
                                    ins.location.x - block.base_point.x,
                                    ins.location.y - block.base_point.y,
                                )
                                .scaling(
                                    self.scale_fact.x * ins.x_scale_factor,
                                    self.scale_fact.y * ins.y_scale_factor,
                                )
                                .blocks(self.blocks)
                                .build()
                                .ok()
                        })
                        .collect(),
                ))
            }
            EntityType::Leader(leader) => {
                let ld: Leader = leader.into();

                Ok(Objects::Group(
                    ld.0.into_iter()
                        .map(|mut ln| {
                            ln.scale(self.scale_fact.x, self.scale_fact.y);

                            ln.x1 += self.offset.x;
                            ln.y1 -= self.offset.y;

                            ln.x2 += self.offset.x;
                            ln.y2 -= self.offset.y;

                            Objects::Line(ln)
                        })
                        .collect(),
                ))
            }
            EntityType::AttributeDefinition(attrib) => Ok({
                //need to look up the proper way to get the color for the Attrib
                let mut dtext = DTextBuilder::from_attrib(attrib)
                    .color(HexColor::from_u32(self.ent.common.color_24_bit as u32))
                    .build();

                dtext.scale(self.scale_fact.x, self.scale_fact.y);

                dtext.x += self.offset.x;
                dtext.y -= self.offset.y;

                Objects::DynamicText(dtext)
            }),
            _ => {
                //dbg!(&self.ent.specific);
                Err("Need to implement the rest of the entity types")
            }
        }
    }
}

impl From<&Objects> for Either<XMLElement, Vec<XMLElement>> {
    fn from(obj: &Objects) -> Self {
        match obj {
            Objects::Arc(arc) => Either::Left(arc.into()),
            Objects::Ellipse(ell) => Either::Left(ell.into()),
            Objects::Polygon(poly) => Either::Left(poly.into()),
            Objects::Rectangle(rect) => Either::Left(rect.into()),
            Objects::DynamicText(dtext) => Either::Left(dtext.into()),
            Objects::Text(txt) => Either::Left(txt.into()),
            Objects::Line(line) => Either::Left(line.into()),
            Objects::Group(block) => Either::Right(
                block
                    .iter()
                    //.flatten()
                    .filter_map(|obj| XMLElement::try_from(obj).ok())
                    .collect(),
            ),
        }
    }
}

impl TryFrom<&Objects> for XMLElement {
    type Error = &'static str; // add better error later

    fn try_from(obj: &Objects) -> Result<Self, Self::Error> {
        match obj {
            Objects::Arc(arc) => Ok(arc.into()),
            Objects::Ellipse(ell) => Ok(ell.into()),
            Objects::Polygon(poly) => Ok(poly.into()),
            Objects::Rectangle(rect) => Ok(rect.into()),
            Objects::DynamicText(dtext) => Ok(dtext.into()),
            Objects::Text(txt) => Ok(txt.into()),
            Objects::Line(line) => Ok(line.into()),
            Objects::Group(_) => Err("Unsupported"),
        }
    }
}

#[derive(Debug)]
pub struct Description {
    objects: Vec<Objects>,
}

impl Bounding for Description {
    fn left_bound(&self) -> f64 {
        let lb = self.objects.iter().min_by(|ob1, ob2| {
            ob1.left_bound()
                .partial_cmp(&ob2.left_bound())
                .unwrap_or(Ordering::Greater)
        });

        if let Some(lb) = lb {
            lb.left_bound()
        } else {
            0.0
        }
    }

    fn right_bound(&self) -> f64 {
        let rb = self.objects.iter().max_by(|ob1, ob2| {
            ob1.right_bound()
                .partial_cmp(&ob2.right_bound())
                .unwrap_or(Ordering::Less)
        });

        if let Some(rb) = rb {
            rb.left_bound()
        } else {
            0.0
        }
    }

    fn top_bound(&self) -> f64 {
        let tb = self.objects.iter().min_by(|ob1, ob2| {
            ob1.top_bound()
                .partial_cmp(&ob2.top_bound())
                .unwrap_or(Ordering::Greater)
        });

        if let Some(tb) = tb {
            tb.top_bound()
        } else {
            0.0
        }
    }

    fn bot_bound(&self) -> f64 {
        let bb = self.objects.iter().max_by(|ob1, ob2| {
            ob1.bot_bound()
                .partial_cmp(&ob2.bot_bound())
                .unwrap_or(Ordering::Less)
        });

        if let Some(bb) = bb {
            bb.top_bound()
        } else {
            0.0
        }
    }
}

impl ScaleEntity for Description {
    fn scale(&mut self, fact_x: f64, fact_y: f64) {
        self.objects
            .iter_mut()
            .for_each(|ob| ob.scale(fact_x, fact_y));
    }
}

impl From<&Description> for XMLElement {
    fn from(desc: &Description) -> Self {
        let mut desc_xml = XMLElement::new("description");
        for obj in &desc.objects {
            if let Ok(elem) = XMLElement::try_from(obj) {
                desc_xml.add_child(elem);
            }
            for obj in obj.descendants() {
                if let Ok(elem) = XMLElement::try_from(obj) {
                    desc_xml.add_child(elem);
                }
            }
        }
        desc_xml
    }
}

/*impl TryFrom<Drawing> for Description {
    type Error = &'static str; //add better error later

    fn try_from(drw: Drawing) -> Result<Self, Self::Error> {
        drw.entities().filter_map(|ent| Objects::try_from(ent).ok()).collect();
    }
}*/
impl From<(&Drawing, Option<f64>)> for Description {
    fn from((drw, spline_step): (&Drawing, Option<f64>)) -> Self {
        let _from_drw_span = span!(Level::TRACE, "Converting Drawing to Description");

        Self {
            objects: drw
                .entities()
                .filter_map(|ent| match &ent.specific {
                    EntityType::Insert(ins) => {
                        let block = find_block(drw, &ins.name)?;
                        let blocks: Vec<&Block> = drw.blocks().collect();
                        trace!(
                            "Creating Group from block {}. Pos(x:{}, y:{}). Scale(x:{}, y:{})",
                            ins.name,
                            ins.location.x,
                            ins.location.y,
                            ins.x_scale_factor,
                            ins.y_scale_factor
                        );
                        Some(Objects::Group(
                            block
                                .entities
                                .iter()
                                .filter_map(|ent| {
                                    ObjectsBuilder::new(ent, spline_step)
                                        //very confused here, in one test file if I leave out the ins locations here it puts things in the
                                        //wrong location, and puts them in the correct location when I add the ins location in.
                                        //but in another file it's the opposite, not sure why the difference...
                                        .offsets(ins.location.x, ins.location.y)
                                        .scaling(ins.x_scale_factor, ins.y_scale_factor)
                                        .blocks(&blocks)
                                        .build()
                                        .ok()
                                })
                                .collect(),
                        ))
                    }
                    _ => ObjectsBuilder::new(ent, spline_step).build().ok(),
                })
                .collect(),
        }
    }
}

//probably don't need to worry about this as they won't exist in the dxf...
/*pub struct Terminal {
    x: f64,
    y: f64,
    uuid: Uuid,
    name: String,
    orientation: TermOrient,
    //type?
    //  Generic
    //  Indoor Terminal Block
    //  External Terminal Block
}*/

#[derive(Debug)]
pub struct Names {
    names: Vec<Name>,
}

impl From<&Names> for XMLElement {
    fn from(nme: &Names) -> Self {
        let mut names_elmt = XMLElement::new("names");
        for name in &nme.names {
            let mut nm_elmt = XMLElement::new("name");
            nm_elmt.add_attribute("lang", &name.lang);
            nm_elmt.add_text(&name.value);
            names_elmt.add_child(nm_elmt);
        }
        names_elmt
    }
}

#[derive(Debug)]
pub struct Name {
    lang: String, //should this be an enum of language shorts at some point, maybe not worth it
    value: String,
}

#[derive(Debug)]
pub struct ElmtUuid {
    uuid: Uuid,
}

impl From<Uuid> for ElmtUuid {
    fn from(uuid: Uuid) -> Self {
        ElmtUuid { uuid }
    }
}

impl From<&ElmtUuid> for XMLElement {
    fn from(uuid: &ElmtUuid) -> Self {
        let mut uuid_xml = XMLElement::new("uuid");
        uuid_xml.add_attribute("uuid", format!("{{{}}}", uuid.uuid));
        uuid_xml
    }
}

//I need to check what these other item types are used for. I think it's unlikely
//I'll ever need for this tool, so it might be worth just hard coding the "element"
//string when writing out the XML, but for now I'll just comment out the other enum
//variants to suppress the clippy warnings.
#[derive(Debug)]
enum ItemType {
    Element = 1,
    /*ElementsCategory = 2,
    ElementsCollection = 4,
    ElementsContainer = 6,
    ElementsCollectionItem = 7,
    TitleBlockTemplate = 8,
    TitleBlockTemplatesCollection = 16,
    TitleBlockTemplatesCollectionItem = 24,
    Diagram = 32,
    Project = 64,
    All = 127,*/
}

impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Element => "element",
                /*Self::ElementsCategory | Self::ElementsContainer | Self::ElementsCollectionItem =>
                    "elements category",
                Self::ElementsCollection => "element",
                Self::TitleBlockTemplate | Self::TitleBlockTemplatesCollectionItem =>
                    "title block template",
                Self::TitleBlockTemplatesCollection => "title block templates collection",
                Self::Diagram => "diagram",
                Self::Project => "project",
                Self::All => "All",*/
            }
        )
    }
}

#[derive(Debug)]
enum HAlignment {
    Left,
    Center,
    Right,
}

impl Display for HAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Left => "AlignLeft",
                Self::Center => "AlignHCenter",
                Self::Right => "AlignRight",
            }
        )
    }
}

impl From<AttachmentPoint> for HAlignment {
    fn from(value: AttachmentPoint) -> Self {
        match value {
            AttachmentPoint::TopLeft
            | AttachmentPoint::MiddleLeft
            | AttachmentPoint::BottomLeft => HAlignment::Left,
            AttachmentPoint::TopCenter
            | AttachmentPoint::MiddleCenter
            | AttachmentPoint::BottomCenter => HAlignment::Center,
            AttachmentPoint::TopRight
            | AttachmentPoint::MiddleRight
            | AttachmentPoint::BottomRight => HAlignment::Right,
        }
    }
}

impl From<HorizontalTextJustification> for HAlignment {
    fn from(value: HorizontalTextJustification) -> Self {
        //https://ezdxf.readthedocs.io/en/stable/tutorials/text.html#tut-text
        match value {
            HorizontalTextJustification::Left => HAlignment::Left,
            HorizontalTextJustification::Center => HAlignment::Center,
            HorizontalTextJustification::Right => HAlignment::Right,

            //TODO: Handling the Aligned Middle and Fit alignments are a bit more complicated
            //for now I'll just default if it gets one of those we Align Left
            _ => HAlignment::Left,
        }
    }
}

#[derive(Debug)]
enum VAlignment {
    Top,
    Center,
    Bottom,
}

impl Display for VAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Top => "AlignTop",
                Self::Center => "AlignVCenter",
                Self::Bottom => "AlignBottom",
            }
        )
    }
}

impl From<AttachmentPoint> for VAlignment {
    fn from(value: AttachmentPoint) -> Self {
        match value {
            AttachmentPoint::TopLeft | AttachmentPoint::TopCenter | AttachmentPoint::TopRight => {
                VAlignment::Top
            }
            AttachmentPoint::MiddleLeft
            | AttachmentPoint::MiddleCenter
            | AttachmentPoint::MiddleRight => VAlignment::Center,
            AttachmentPoint::BottomLeft
            | AttachmentPoint::BottomCenter
            | AttachmentPoint::BottomRight => VAlignment::Bottom,
        }
    }
}

impl From<VerticalTextJustification> for VAlignment {
    fn from(value: VerticalTextJustification) -> Self {
        //https://ezdxf.readthedocs.io/en/stable/tutorials/text.html#tut-text
        match value {
            VerticalTextJustification::Top => VAlignment::Top,
            VerticalTextJustification::Middle => VAlignment::Center,
            VerticalTextJustification::Baseline | VerticalTextJustification::Bottom => {
                VAlignment::Bottom
            }
        }
    }
}

#[derive(Debug)]
enum LineEnd {
    None,
    SimpleArrow,
    /*TriangleArrow,
    Circle,
    Diamond,*/
}

impl Display for LineEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "none",
                Self::SimpleArrow => "simple",
                /*Self::TriangleArrow => "triangle",
                Self::Circle => "circle",
                Self::Diamond => "diamond",*/
            }
        )
    }
}

#[derive(Debug)]
enum LinkType {
    Simple,
    /*Master,
    Slave,
    NextReport,
    PrevReport,
    TermBlock,
    Thumbnail,*/
}

impl Display for LinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Simple => "simple",
                /*Self::Master => "master",
                Self::Slave => "slave",
                Self::NextReport => "next_report",
                Self::PrevReport => "previous_report",
                Self::TermBlock => "terminal",
                Self::Thumbnail => "thumbnail",*/
            }
        )
    }
}

#[derive(Debug)]
pub struct ElemInfos {
    elem_info: Vec<ElemInfo>,
}

impl From<&ElemInfos> for XMLElement {
    fn from(elems: &ElemInfos) -> Self {
        let mut elems_xml = XMLElement::new("elementInformations");
        for elem in &elems.elem_info {
            elems_xml.add_child(elem.into());
        }

        elems_xml
    }
}

#[derive(Debug)]
pub struct ElemInfo {
    //there seems to be a list in the editor with the following values (per the XML)
    //  * supplier
    //  * description
    //  * machine_manufacturer_reference
    //  * manufacturer_reference
    //  * quantity
    //  * manufacturer
    //  * label
    //  * unity
    //  * plant
    //  * comment
    //  * designation
    // But can it only ever be these values? Might need to dig into the code. For now I'll use a string
    name: String,

    //I would assume show would be a bool...but instead of a true value I'm getting a "1"  in the XML
    //generated by the element editor. Maybe this means something else? I'll use an i32 for now
    show: i32,

    value: String,
}

impl From<&ElemInfo> for XMLElement {
    fn from(elem: &ElemInfo) -> Self {
        let mut elem_xml = XMLElement::new("elementInformation");
        elem_xml.add_attribute("show", elem.show);
        elem_xml.add_attribute("name", &elem.name);
        elem_xml.add_text(&elem.value);

        elem_xml
    }
}

#[inline]
pub fn two_dec(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}

//Should be the relevant Qt5 Code for the font string in Qt5...
//Might need to look it up for Qt6, since it appears to have changed
//and add in support for either or?

/*https://codebrowser.dev/qt5/qtbase/src/gui/text/qfont.cpp.html
/*!
    Returns a description of the font. The description is a
    comma-separated list of the attributes, perfectly suited for use
    in QSettings, and consists of the following:
    \list
    \li Font family
    \li Point size
    \li Pixel size
    \li Style hint
    \li Font weight
    \li Font style
    \li Underline
    \li Strike out
    \li Fixed pitch
    \li Always \e{0}
    \li Capitalization
    \li Letter spacing
    \li Word spacing
    \li Stretch
    \li Style strategy
    \li Font style (omitted when unavailable)
    \endlist
    \sa fromString()
 */
QString QFont::toString() const
{
    const QChar comma(QLatin1Char(','));
    QString fontDescription = family() + comma +
        QString::number(     pointSizeF()) + comma +
        QString::number(      pixelSize()) + comma +
        QString::number((int) styleHint()) + comma +
        QString::number(         weight()) + comma +
        QString::number((int)     style()) + comma +
        QString::number((int) underline()) + comma +
        QString::number((int) strikeOut()) + comma +
        QString::number((int)fixedPitch()) + comma +
        QString::number((int)   false);
    QString fontStyle = styleName();
    if (!fontStyle.isEmpty())
        fontDescription += comma + fontStyle;
    return fontDescription;
}
    */

#[derive(Debug)]
pub enum FontStyleHint {
    Helvetica,
    Times,
    Courier,
    OldEnglish,
    System,
    AnyStyle,
    Cursive,
    Monospace,
    Fantasy,
}

/*impl FontStyleHint {
    pub const SansSerif: FontStyleHint = FontStyleHint::Helvetica;
    pub const Serif: FontStyleHint = FontStyleHint::Times;
    pub const TypeWriter: FontStyleHint = FontStyleHint::Courier;
    pub const Decorative: FontStyleHint = FontStyleHint::OldEnglish;
}
*/

impl From<&FontStyleHint> for i32 {
    fn from(value: &FontStyleHint) -> Self {
        match value {
            FontStyleHint::Helvetica => 0,
            FontStyleHint::Times => 1,
            FontStyleHint::Courier => 2,
            FontStyleHint::OldEnglish => 3,
            FontStyleHint::System => 4,
            FontStyleHint::AnyStyle => 5,
            FontStyleHint::Cursive => 6,
            FontStyleHint::Monospace => 7,
            FontStyleHint::Fantasy => 8,
        }
    }
}

#[derive(Debug)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl From<&FontStyle> for i32 {
    fn from(value: &FontStyle) -> Self {
        match value {
            FontStyle::Normal => 0,
            FontStyle::Italic => 1,
            FontStyle::Oblique => 2,
        }
    }
}

#[derive(Debug)]
struct FontInfo {
    family: String,
    point_size: f64,
    pixel_size: i32,
    style_hint: FontStyleHint,
    weight: i32,
    style: FontStyle,
    underline: bool,
    strike_out: bool,
    fixed_pitch: bool,
    style_name: Option<String>,
}

impl Default for FontInfo {
    fn default() -> Self {
        //Might want to revisit these defaults
        //but I'll put something in for now
        Self {
            family: "osifont".into(),
            point_size: 12.0,
            pixel_size: i32::default(),
            style_hint: FontStyleHint::Helvetica,
            weight: i32::default(),
            style: FontStyle::Normal,
            underline: false,
            strike_out: false,
            fixed_pitch: false,
            style_name: None,
        }
    }
}

impl Display for FontInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{},{},0{}",
            self.family,
            self.point_size.round(),
            self.pixel_size,
            Into::<i32>::into(&self.style_hint),
            self.weight,
            Into::<i32>::into(&self.style),
            i32::from(self.underline),
            i32::from(self.strike_out),
            i32::from(self.fixed_pitch),
            if let Some(sn) = &self.style_name {
                format!(",{sn}")
            } else {
                String::new()
            },
        )
    }
}

#[derive(Debug)]
enum TextEntity<'a> {
    Text(&'a dxf::entities::Text),
    MText(&'a dxf::entities::MText),
    Attrib(&'a AttributeDefinition),
}
