use std::io::{stdin, Write};

mod floor_plan;

fn main() {
    let  pic_path="img/simple_floor_plan_1.png";
    let mut floor_plan = floor_plan::Area::new(pic_path, 20);
    let a= floor_plan.mean_pixel_is_smaller_than_tolerance_if_last_was_mismatch([255, 255, 255, 255], [166, 167, 162, 162], [89, 87, 86, 89]);
    let b= floor_plan.mean_pixel_is_smaller_than_tolerance_if_last_was_mismatch([89, 87, 87, 89], [166, 167, 162, 162], [255, 255, 255, 255]);
    let c= floor_plan.mean_pixel_is_smaller_than_tolerance_if_last_was_mismatch([2, 0, 0, 0], [166, 167, 162, 162], [0, 0, 0, 0]);
    println!("a: {}, b: {}, c: {}", a, b, c);
}

