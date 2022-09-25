use std::fs::File;
use std::io::Write;
use image::Rgba;

mod html_box;
mod FloorPlan;

fn main() {
    let mut html = html_box::Html::new("img/index2.Html", 5);
    html.write_pixel_at(0, 0, 0xFFFF00);
    html.write_pixel_at(1, 0, 0xFF00FF);
    html.close();
}

