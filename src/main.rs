use binpack2d_test::benchmarks;
use binpack2d_test::bin_svg;

fn main() {
    // benchmarks();
    export_svg("images");
}

#[allow(dead_code)]
fn export_svg(folder: &str) {
    // Use online SVG compressor to reduce file size (https://vecta.io/nano)
    if let Err(e) = bin_svg::export_maxrects(folder) {
        println!("Error: {e:?}");
    }
    if let Err(e) = bin_svg::export_guillotine(folder) {
        println!("Error: {e:?}");
    }
}

#[allow(dead_code)]
fn benchmark() {
    benchmarks::benchmark_maxrects();
    benchmarks::benchmark_guillotine();
}
