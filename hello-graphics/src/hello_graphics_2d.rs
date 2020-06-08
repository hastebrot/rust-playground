extern crate piet;
extern crate piet_common;

use piet::{Color, RenderContext};
use piet::kurbo::Line;
use piet_common::Device;

fn main() {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 200;
    const HIDPI: f64 = 2.0;

    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(WIDTH, HEIGHT, HIDPI).unwrap();
    let mut rc = bitmap.render_context();

    rc.clear(Color::WHITE);
    let brush = rc.solid_brush(Color::rgb8(0x00, 0x00, 0x80));
    rc.stroke(Line::new((10.0, 10.0), (100.0, 50.0)), &brush, 1.0);

    rc.finish().unwrap();
    std::mem::drop(rc);

    let path = format!("sample-{}.png", 1);
    bitmap.save_to_file(&path).expect("file save error");
}
