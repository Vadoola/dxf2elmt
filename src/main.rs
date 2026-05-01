#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

extern crate dxf;
extern crate simple_xml_builder;
extern crate unicode_segmentation;

use anyhow::{Context, Ok, Result};
use clap::Parser;
use dxf::entities::EntityType;
use dxf::Drawing;
use qelmt::Definition;
//use rayon::prelude::*;
use simple_xml_builder::XMLElement;
use std::time::Instant;
use std::{io, path::PathBuf};
use tracing::{span, trace, warn, Level};
use tracing_subscriber::prelude::*;

#[cfg(feature = "venator")]
use venator::Venator;

mod qelmt;

#[derive(Parser, Debug)]
#[command(name = "dxf2elmt")]
#[command(author, version, about = "A CLI program to convert .dxf files into .elmt files", long_about = None)]
struct Args {
    /// The .dxf file to convert
    //#[clap(short, long, value_parser)]
    file_names: Vec<PathBuf>,

    /// Activates verbose output, eliminates .elmt file writing
    #[clap(short, long, value_parser, default_value_t = false)]
    verbose: bool,

    /// Converts text entities into dynamic text instead of the default text box
    #[clap(short, long, value_parser, default_value_t = false)]
    dtext: bool,

    /// Determine the number of lines you want each spline to have (more lines = greater resolution). By default the number of lines
    /// will be calculated based on the control points and distance they are from the curve, but can be overriden by setting this value
    /*#[clap(short, long, value_parser, default_value_t = 20)]
    spline_step: u32,*/
    #[clap(short, long)]
    spline_step: Option<f64>,

    /// Toggles information output... defaults to off
    #[clap(short, long, value_parser, default_value_t = false)]
    info: bool,
}

pub mod file_writer;

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    #[cfg(feature = "venator")]
    let tr_reg = tracing_subscriber::registry()
        .with(Venator::default())
        .with(tracing_subscriber::fmt::Layer::default());

    #[cfg(not(feature = "venator"))]
    let tr_reg = {
        let file_layer =
            if let std::result::Result::Ok(file) = std::fs::File::create("dxf2elmt.log") {
                //if we can create a log file use it
                Some(tracing_subscriber::fmt::layer().with_writer(std::sync::Arc::new(file)))
            } else {
                None
            };

        let stde_layer = tracing_subscriber::fmt::layer()
            .pretty()
            .with_writer(io::stderr);
        tracing_subscriber::registry()
            .with(file_layer.map(|fl| {
                fl.with_file(true)
                    .with_line_number(true)
                    .with_thread_ids(true)
                    .with_ansi(false)
                    .with_filter(tracing_subscriber::EnvFilter::from_env("DXF2E_LOG"))
            }))
            .with(
                stde_layer
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_ids(true)
                    .with_filter(tracing_subscriber::EnvFilter::from_env("DXF2E_LOG")),
            )
    };
    tr_reg.init();

    trace!("Starting dxf2elmt");

    // Start recording time
    let now: Instant = Instant::now();

    // Collect arguments
    let args: Args = Args::parse_from(wild::args());

    // Load dxf file
    let dxf_loop_span = span!(Level::TRACE, "Looping over dxf files");
    let dxf_loop_guard = dxf_loop_span.enter();
    for file_name in args.file_names {
        let friendly_file_name = file_name
            .file_stem()
            .unwrap_or_else(|| file_name.as_os_str())
            .to_string_lossy();
        let drawing: Drawing = Drawing::load_file(&file_name).context(format!(
            "Failed to load {friendly_file_name}...\n\tMake sure the file is a valid .dxf file.",
        ))?;
        let q_elmt = if let Some(spline_step) = args.spline_step {
            Definition::new_with_step(friendly_file_name.clone(), spline_step, &drawing)
        } else {
            Definition::new(friendly_file_name.clone(), &drawing)
        };

        if !args.verbose && args.info {
            println!("{friendly_file_name} loaded...");
        }

        // Initialize counts
        let mut circle_count: u32 = 0;
        let mut line_count: u32 = 0;
        let mut arc_count: u32 = 0;
        let mut spline_count: u32 = 0;
        let mut text_count: u32 = 0;
        let mut ellipse_count: u32 = 0;
        let mut polyline_count: u32 = 0;
        let mut lwpolyline_count: u32 = 0;
        let mut solid_count: u32 = 0;
        let mut block_count: u32 = 0;
        let mut other_count: u32 = 0;

        // Loop through all entities, counting the element types
        //drawing.entities().for_each(|e| match e.specific {
        drawing.entities().for_each(|e| match e.specific {
            EntityType::Circle(ref _circle) => {
                circle_count += 1;
            }
            EntityType::Line(ref _line) => {
                line_count += 1;
            }
            EntityType::Arc(ref _arc) => {
                arc_count += 1;
            }
            EntityType::Spline(ref _spline) => {
                spline_count += 1;
            }
            EntityType::Text(ref _text) => {
                text_count += 1;
            }
            EntityType::Ellipse(ref _ellipse) => {
                ellipse_count += 1;
            }
            EntityType::Polyline(ref _polyline) => {
                polyline_count += 1;
            }
            EntityType::LwPolyline(ref _lwpolyline) => {
                lwpolyline_count += 1;
            }
            EntityType::Solid(ref _solid) => {
                solid_count += 1;
            }
            EntityType::Insert(ref _insert) => {
                block_count += 1;
            }
            _ => {
                other_count += 1;
            }
        });

        // Create output file for .elmt
        let out_file = file_writer::create_file(args.verbose, args.info, &file_name)?;

        // Write to output file
        let out_xml = XMLElement::from(&q_elmt);
        out_xml
            .write(&out_file)
            .context("Failed to write output file.")?;

        if args.info {
            println!("Conversion complete!\n");

            // Print stats
            println!("STATS");
            println!("~~~~~~~~~~~~~~~");
            println!("Circles: {circle_count}");
            println!("Lines: {line_count}");
            println!("Arcs: {arc_count}");
            println!("Splines: {spline_count}");
            println!("Texts: {text_count}");
            println!("Ellipses: {ellipse_count}");
            println!("Polylines: {polyline_count}");
            println!("LwPolylines: {lwpolyline_count}");
            println!("Solids: {solid_count}");
            println!("Blocks: {block_count}");
            println!("Currently Unsupported: {other_count}");

            println!("\nTime Elapsed: {} ms", now.elapsed().as_millis());
        }

        if args.verbose {
            print!("{out_xml}");
        }
    }
    drop(dxf_loop_guard);

    Ok(())
}
