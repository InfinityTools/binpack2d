use binpack2d::guillotine::GuillotineBin;
use binpack2d::maxrects::MaxRectsBin;
use binpack2d::{bin_new, BinPacker, BinType, Dimension, Rectangle};
use rand::prelude::{Rng, SeedableRng, StdRng};
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;
use svg_fmt::{Align, BeginSvg, black, EndSvg, Fill, rectangle, rgb, Stroke, text, white};
use crate::{GUILLOTINE_CHOICES, GUILLOTINE_METHODS, MAXRECTS_RULES, prepare_nodes_list};

/// Exports visualization of packed MaxRects bins as SVG graphics.
pub fn export_maxrects(folder: &str) -> Result<(), Error> {
    let folder = create_dir(folder)?;

    let width: i32 = 1024;
    let height: i32 = width / 2;

    let nodes = prepare_nodes_list(width, height);

    let mut bin = MaxRectsBin::with_capacity(width, height, nodes.len());

    println!("Generating bins...");
    for rule in MAXRECTS_RULES {
        bin.clear_with(nodes.len());

        let (inserted, _) = bin.insert_list(&nodes, *rule);
        println!(
            "{} of {} nodes inserted into {width}x{height} bin (rule: {rule:?}, occupancy: {:.2})",
            inserted.len(),
            nodes.len(),
            bin.occupancy() * 100.0
        );

        // exporting as svg
        let file_name = format!("{folder}/MaxRects_{rule:?}.svg");
        let mut f = File::create(&file_name)?;
        println!("Creating file {file_name}...");

        // canvas size
        writeln!(
            f,
            "{}",
            BeginSvg {
                w: bin.width() as f32,
                h: (bin.height() + 24) as f32
            }
        )?;

        // bin
        writeln!(
            f,
            "    {}",
            rectangle(0.0, 0.0, bin.width() as f32, bin.height() as f32)
                .fill(Fill::Color(white()))
                .stroke(Stroke::Color(black(), 1.0))
        )?;

        // rectangles
        let mut min_size: i32 = i32::MAX;
        let mut max_size: i32 = i32::MIN;
        for (i, rect) in inserted.iter().enumerate() {
            min_size = min_size.min(rect.width().min(rect.height()));
            max_size = max_size.max(rect.width().max(rect.height()));
            writeln!(f, "    {}", svg_rectangle(rect, i, inserted.len()))?;
        }

        // statistics
        let info = format!("MaxRectsBin(size: {}x{}, rule: {rule:?}): inserted {} of {} rectangles with w/h lengths between [{min_size}, {max_size}], occupancy: {:.2}%",
                           bin.width(), bin.height(), inserted.len(), nodes.len(), bin.occupancy() * 100.0);
        writeln!(
            f,
            "    {}",
            text(0.0, bin.height() as f32, info)
                .size(12.0)
                .color(black())
                .offset(0.0, 14.0)
                .align(Align::Left)
        )?;

        writeln!(f, "{}", EndSvg)?;
    }

    Ok(())
}

/// Exports visualization of packed Guillotine bins as SVG graphics.
pub fn export_guillotine(folder: &str) -> Result<(), Error> {
    let folder = create_dir(folder)?;

    let width: i32 = 1024;
    let height: i32 = width / 2;

    let nodes = prepare_nodes_list(width, height);

    let mut bin = GuillotineBin::with_capacity(width, height, nodes.len());

    println!("Generating bins...");
    for choice in GUILLOTINE_CHOICES {
        for method in GUILLOTINE_METHODS {
            bin.clear_with(nodes.len());

            let (inserted, _) = bin.insert_list(&nodes, true, *choice, *method);
            println!("{} of {} nodes inserted into {width}x{height} bin (choice: {choice:?}, method: {method:?}, occupancy: {:.2})",
                     inserted.len(), nodes.len(), bin.occupancy() * 100.0);

            // exporting as svg
            let file_name = format!("{folder}/Guillotine_{choice:?}_{method:?}.svg");
            let mut f = File::create(&file_name)?;
            println!("Creating file {file_name}...");
            writeln!(
                f,
                "{}",
                BeginSvg {
                    w: bin.width() as f32,
                    h: (bin.height() + 24) as f32
                }
            )?;

            // bin
            writeln!(
                f,
                "    {}",
                rectangle(0.0, 0.0, bin.width() as f32, bin.height() as f32)
                    .fill(Fill::Color(white()))
                    .stroke(Stroke::Color(black(), 1.0))
            )?;

            // rectangles
            let mut min_size: i32 = i32::MAX;
            let mut max_size: i32 = i32::MIN;
            for (i, rect) in inserted.iter().enumerate() {
                min_size = min_size.min(rect.width().min(rect.height()));
                max_size = max_size.max(rect.width().max(rect.height()));
                writeln!(f, "    {}", svg_rectangle(rect, i, inserted.len()))?;
            }

            // statistics
            let info = format!("MaxRectsBin(size: {}x{}, choice: {choice:?}, method: {method:?}): inserted {} of {} rectangles with w/h lengths between [{min_size}, {max_size}], occupancy: {:.2}%",
                               bin.width(), bin.height(), inserted.len(), nodes.len(), bin.occupancy() * 100.0);
            writeln!(
                f,
                "    {}",
                text(0.0, bin.height() as f32, info)
                    .size(12.0)
                    .color(black())
                    .offset(0.0, 14.0)
                    .align(Align::Left)
            )?;

            writeln!(f, "{}", EndSvg)?;
        }
    }

    Ok(())
}

// Generates a Svg rectangle command based on the parameters and returns it as `String`.
fn svg_rectangle(rect: &Rectangle, index: usize, count: usize) -> String {
    let range = 192.0 * 3.0;
    let range_r = range / 3.0;
    let range_g = 0f32;
    let range_b = 2.0 * (range / 3.0);
    let c = index as f32 / count as f32 * range;

    let r = (c - range_r).min(192.0) as u8 + 48;
    let g = (c - range_g).min(192.0) as u8 + 48;
    let b = (c - range_b).min(192.0) as u8 + 48;

    // let c = (48.0 + (index as f32 / count as f32) * 192.0) as u8;
    let x = (rect.x() + 1) as f32;
    let y = (rect.y() + 1) as f32;
    let w = (rect.width() - 1) as f32;
    let h = (rect.height() - 1) as f32;
    format!(
        "{}",
        rectangle(x, y, w, h)
            .fill(Fill::Color(rgb(r, g, b)))
            .stroke(Stroke::Color(rgb(r / 2, g / 2, b / 2), 1.0))
    )
}

/// Creates a folder of given name and returns success state.
fn create_dir(folder: &str) -> Result<String, Error> {
    let folder = if folder.is_empty() { String::from(".") } else { folder.to_string() };

    if !Path::new(&folder).is_dir() {
        std::fs::create_dir(&folder)?;
    }

    Ok(folder)
}

/// Visualizes a bin pack operation in the console output.
pub fn binpack_console() {
    let height: i32 = 32;
    let width: i32 = height * 2;
    let size: usize = 62;

    let size_min: u32 = (width.min(height) as u32 / 16).max(2);
    let size_max: u32 = width.min(height) as u32 / 2;

    let mut rng = StdRng::seed_from_u64(123456789);
    let mut nodes = Vec::with_capacity(size);
    for i in 0..size {
        nodes.push(Dimension::with_id(
            i as isize,
            rng.gen_range(size_min..size_max) as i32,
            rng.gen_range(size_min..size_max) as i32,
            1,
        ));
    }

    for bin_type in &[BinType::MaxRects, BinType::Guillotine] {
        let mut bin = bin_new(*bin_type, width, height);
        bin.insert_list(&nodes);
        println!(
            "{bin_type:?}: {} of {} nodes inserted into {width}x{height} bin (occupancy: {})",
            bin.len(),
            nodes.len(),
            bin.occupancy()
        );
        println!("{}", bin.visualize());
        println!();
    }
}
