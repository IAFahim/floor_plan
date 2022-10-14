use std::io::{stdin, Write};

mod floor_plan;

fn main() {
    let  pic_path="img/simple_floor_plan_2.jpg";
    let mut floor_plan = floor_plan::Area::new(pic_path, 20);
    floor_plan.get_rid_of_all_light_color();
}

