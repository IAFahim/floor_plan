use std::io::{stdin, Write};

mod floor_plan;

fn main() {
    let  pic_path="img/simple_floor_plan_1.png";
    let mut floor_plan = floor_plan::Area::new(pic_path, 20);
    floor_plan.y_axis_boundary_down_to_up();
    println!("{:?}", floor_plan.y_boundary_bottom_up);
    println!("{:?}", floor_plan.y_boundary_bottom_up.get(2070).unwrap().1);
    floor_plan.y_axis_heat_map();

}

